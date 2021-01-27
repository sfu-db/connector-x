use ndarray::{Array, ArrayView, ArrayViewMut, Axis, Dimension, Ix, NdIndex};
use std::any::Any;
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
    A: Send + Sync,
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
}

impl<'a, A, D> AnyArrayViewMut<'a, D> for ArrayViewMut<'a, A, D>
where
    A: Send,
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
}
