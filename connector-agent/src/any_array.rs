use ndarray::{Array, ArrayView, ArrayViewMut, Axis, Dimension, Ix};
use std::any::{Any, TypeId};
use std::mem::transmute;

trait AnyArrayObject<D> {
    fn view_mut<'a>(&'a mut self) -> Box<dyn ArrayViewMutObject<'a, D> + 'a>;
    fn view<'a>(&'a self) -> Box<dyn ArrayViewObject<'a, D> + 'a>;
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

impl<A, D> AnyArrayObject<D> for Array<A, D>
where
    A: 'static + Send,
    D: 'static + Dimension,
{
    fn view<'a>(&'a self) -> Box<dyn ArrayViewObject<'a, D> + 'a> {
        Box::new(Array::<A, D>::view(self))
    }

    fn view_mut<'a>(&'a mut self) -> Box<dyn ArrayViewMutObject<'a, D> + 'a> {
        Box::new(Array::<A, D>::view_mut(self))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct AnyArray<D> {
    inner: Box<dyn AnyArrayObject<D>>,
    elem_type: TypeId,
}

impl<D> AnyArray<D>
where
    D: 'static + Dimension,
{
    pub fn new<A>(array: Array<A, D>) -> Self
    where
        A: 'static + Send,
    {
        Self {
            inner: Box::new(array),
            elem_type: TypeId::of::<A>(),
        }
    }

    pub fn view<'a>(&'a self) -> AnyArrayView<'a, D> {
        AnyArrayView {
            inner: self.inner.view(),
            elem_type: self.elem_type,
        }
    }

    pub fn view_mut<'a>(&'a mut self) -> AnyArrayViewMut<'a, D> {
        AnyArrayViewMut {
            inner: self.inner.view_mut(),
            elem_type: self.elem_type,
        }
    }

    pub fn downcast_ref<A>(&self) -> Option<&Array<A, D>>
    where
        A: 'static,
    {
        self.inner.as_any().downcast_ref()
    }

    pub fn downcast_mut<A>(&mut self) -> Option<&mut Array<A, D>>
    where
        A: 'static,
    {
        self.inner.as_mut_any().downcast_mut()
    }
}

impl<A, D> From<Array<A, D>> for AnyArray<D>
where
    A: 'static + Send,
    D: 'static + Dimension,
{
    fn from(value: Array<A, D>) -> Self {
        Self::new(value)
    }
}

trait ArrayViewObject<'a, D> {
    fn split_at(
        self: Box<Self>,
        axis: Axis,
        index: Ix,
    ) -> (
        Box<dyn ArrayViewObject<'a, D> + 'a>,
        Box<dyn ArrayViewObject<'a, D> + 'a>,
    );
}

impl<'a, A, D> ArrayViewObject<'a, D> for ArrayView<'a, A, D>
where
    A: 'static,
    D: Dimension + 'static,
{
    fn split_at(
        self: Box<Self>,
        axis: Axis,
        index: Ix,
    ) -> (
        Box<dyn ArrayViewObject<'a, D> + 'a>,
        Box<dyn ArrayViewObject<'a, D> + 'a>,
    ) {
        let (l, r) = ArrayView::<A, D>::split_at(*self, axis, index);
        (Box::new(l), Box::new(r))
    }
}

pub struct AnyArrayView<'a, D> {
    inner: Box<dyn ArrayViewObject<'a, D> + 'a>,
    elem_type: TypeId,
}

impl<'a, D> AnyArrayView<'a, D>
where
    D: 'static + Dimension,
{
    pub fn new<A>(view: ArrayView<'a, A, D>) -> Self
    where
        A: 'static,
    {
        Self {
            inner: Box::new(view),
            elem_type: TypeId::of::<A>(),
        }
    }

    pub fn downcast<A: 'static>(&self) -> Option<&ArrayView<'a, A, D>> {
        if self.elem_type == TypeId::of::<A>() {
            Some(unsafe { self.udowncast() })
        } else {
            None
        }
    }

    pub unsafe fn udowncast<A: 'static>(&self) -> &ArrayView<'a, A, D> {
        let (data, _vtable): (&ArrayView<A, D>, usize) = transmute(&*self.inner);
        data
    }

    pub fn split_at(self, axis: Axis, index: Ix) -> (AnyArrayView<'a, D>, AnyArrayView<'a, D>) {
        let (l, r) = self.inner.split_at(axis, index);
        (
            AnyArrayView {
                inner: l,
                elem_type: self.elem_type,
            },
            AnyArrayView {
                inner: r,
                elem_type: self.elem_type,
            },
        )
    }
}

trait ArrayViewMutObject<'a, D>: Send {
    fn split_at(
        self: Box<Self>,
        axis: Axis,
        index: Ix,
    ) -> (
        Box<dyn ArrayViewMutObject<'a, D> + 'a>,
        Box<dyn ArrayViewMutObject<'a, D> + 'a>,
    );
}

impl<'a, A, D> ArrayViewMutObject<'a, D> for ArrayViewMut<'a, A, D>
where
    A: 'static + Send,
    D: 'static + Dimension,
{
    fn split_at(
        self: Box<Self>,
        axis: Axis,
        index: Ix,
    ) -> (
        Box<dyn ArrayViewMutObject<'a, D> + 'a>,
        Box<dyn ArrayViewMutObject<'a, D> + 'a>,
    ) {
        let (l, r) = ArrayViewMut::<A, D>::split_at(*self, axis, index);
        (Box::new(l), Box::new(r))
    }
}

pub struct AnyArrayViewMut<'a, D> {
    inner: Box<dyn ArrayViewMutObject<'a, D> + 'a>,
    elem_type: TypeId,
}

impl<'a, D> AnyArrayViewMut<'a, D>
where
    D: 'static + Dimension,
{
    pub fn new<A>(view: ArrayViewMut<'a, A, D>) -> Self
    where
        A: 'static + Send,
    {
        Self {
            inner: Box::new(view),
            elem_type: TypeId::of::<A>(),
        }
    }

    pub fn downcast<A: 'static>(&mut self) -> Option<&mut ArrayViewMut<'a, A, D>> {
        if self.elem_type == TypeId::of::<A>() {
            Some(unsafe { self.udowncast() })
        } else {
            None
        }
    }

    pub unsafe fn udowncast<A: 'static>(&mut self) -> &mut ArrayViewMut<'a, A, D> {
        let (data, _): (&mut ArrayViewMut<A, D>, usize) = transmute(&mut *self.inner);
        data
    }

    pub fn split_at(
        self,
        axis: Axis,
        index: Ix,
    ) -> (AnyArrayViewMut<'a, D>, AnyArrayViewMut<'a, D>) {
        let (l, r) = self.inner.split_at(axis, index);
        (
            AnyArrayViewMut {
                inner: l,
                elem_type: self.elem_type,
            },
            AnyArrayViewMut {
                inner: r,
                elem_type: self.elem_type,
            },
        )
    }
}
