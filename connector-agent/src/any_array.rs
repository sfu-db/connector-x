use ndarray::{Array, ArrayView, ArrayViewMut, Axis, Dimension, Ix, NdIndex};
use std::any::{Any, TypeId};
use std::mem::transmute;

pub trait AnyArray<D>: Send {
    fn view_mut<'a>(&'a mut self) -> Box<dyn AnyArrayViewMut<'a, D> + 'a>;
    fn view<'a>(&'a self) -> Box<dyn AnyArrayView<'a, D> + 'a>;
    fn as_any(&self) -> &dyn Any;
}

pub trait AnyArrayView<'a, D>: Send {
    // fn as_any(&self) -> &dyn Any;
    fn split_at(
        self: Box<Self>,
        axis: Axis,
        index: Ix,
    ) -> (
        Box<dyn AnyArrayView<'a, D> + 'a>,
        Box<dyn AnyArrayView<'a, D> + 'a>,
    );
    unsafe fn uget(&self, index: (usize, usize)) -> &();
    fn type_id(&self) -> TypeId;
}

impl<'a, D> dyn AnyArrayView<'a, D> {
    pub fn uget_checked<A: 'static>(&self, index: (usize, usize)) -> Option<&'a A> {
        if self.type_id() == TypeId::of::<A>() {
            Some(unsafe { transmute(self.uget(index)) })
        } else {
            None
        }
    }
}

pub trait AnyArrayViewMut<'a, D>: Send {
    // fn as_any(&self) -> &dyn Any;
    fn split_at(
        self: Box<Self>,
        axis: Axis,
        index: Ix,
    ) -> (
        Box<dyn AnyArrayViewMut<'a, D> + 'a>,
        Box<dyn AnyArrayViewMut<'a, D> + 'a>,
    );
    unsafe fn uget_mut(&mut self, index: (usize, usize)) -> &mut ();
    fn type_id(&self) -> TypeId;
}

impl<'a, D> dyn AnyArrayViewMut<'a, D> + 'a {
    pub fn uget_mut_checked<A: 'static>(&mut self, index: (usize, usize)) -> Option<&'a mut A> {
        if AnyArrayViewMut::<'a, D>::type_id(self) == TypeId::of::<A>() {
            Some(unsafe { transmute(self.uget_mut(index)) })
        } else {
            None
        }
    }
}

impl<A, D> AnyArray<D> for Array<A, D>
where
    A: 'static + Send + Sync,
    D: 'static + Dimension,
    (usize, usize): NdIndex<D>,
{
    fn view<'a>(&'a self) -> Box<dyn AnyArrayView<'a, D> + 'a> {
        Box::new(Array::<A, D>::view(self))
    }

    fn view_mut<'a>(&'a mut self) -> Box<dyn AnyArrayViewMut<'a, D> + 'a> {
        Box::new(Array::<A, D>::view_mut(self))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<'a, A, D> AnyArrayView<'a, D> for ArrayView<'a, A, D>
where
    A: Send + Sync + 'static,
    D: Dimension + 'static,
    (usize, usize): NdIndex<D>,
{
    fn split_at(
        self: Box<Self>,
        axis: Axis,
        index: Ix,
    ) -> (
        Box<dyn AnyArrayView<'a, D> + 'a>,
        Box<dyn AnyArrayView<'a, D> + 'a>,
    ) {
        let (l, r) = ArrayView::<A, D>::split_at(*self, axis, index);
        (Box::new(l), Box::new(r))
    }

    unsafe fn uget(&self, index: (usize, usize)) -> &() {
        transmute(self.uget(index))
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<A>()
    }
}

impl<'a, A, D> AnyArrayViewMut<'a, D> for ArrayViewMut<'a, A, D>
where
    A: Send + 'static,
    D: Dimension + 'static,
    (usize, usize): NdIndex<D>,
{
    fn split_at(
        self: Box<Self>,
        axis: Axis,
        index: Ix,
    ) -> (
        Box<dyn AnyArrayViewMut<'a, D> + 'a>,
        Box<dyn AnyArrayViewMut<'a, D> + 'a>,
    ) {
        let (l, r) = ArrayViewMut::<A, D>::split_at(*self, axis, index);
        (Box::new(l), Box::new(r))
    }

    unsafe fn uget_mut(&mut self, index: (usize, usize)) -> &mut () {
        transmute(self.uget_mut(index))
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<A>()
    }
}
