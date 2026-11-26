use std::ops::{Deref, DerefMut};
use url::{form_urlencoded, Url};

/// Remove the given parameters from a URL query, leaving every other parameter
/// byte-for-byte as the caller wrote it.
///
/// `Url::query_pairs_mut` cannot be used for this: it serializes the query as
/// `application/x-www-form-urlencoded`, which encodes a space as `+`. Drivers that
/// percent-decode the query (rust-postgres, for one) then read that `+` literally,
/// so `?options=-c statement_timeout=1s` reaches the server as `-c+statement_timeout=1s`.
pub(crate) fn remove_query_params(url: &Url, params: &[&str]) -> Url {
    let mut stripped = url.clone();
    match url.query() {
        None => stripped,
        Some(query) => {
            let kept: Vec<&str> = query
                .split('&')
                .filter(|segment| !segment.is_empty())
                .filter(|segment| {
                    let raw_key = segment.split('=').next().unwrap_or_default();
                    // compare decoded keys, as `query_pairs` would
                    let key = form_urlencoded::parse(raw_key.as_bytes())
                        .next()
                        .map(|(key, _)| key)
                        .unwrap_or_default();
                    !params.contains(&&*key)
                })
                .collect();

            let query = kept.join("&");
            stripped.set_query(match query.is_empty() {
                true => None,
                false => Some(&query),
            });
            stripped
        }
    }
}

pub struct DummyBox<T>(pub T);

impl<T> Deref for DummyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for DummyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Only the SQLite and Oracle sources need `OwningHandle`.
#[cfg(any(feature = "src_sqlite", feature = "src_oracle"))]
pub(crate) use owning_handle::OwningHandle;

#[cfg(any(feature = "src_sqlite", feature = "src_oracle"))]
mod owning_handle {
    use std::mem::ManuallyDrop;
    use std::ops::{Deref, DerefMut};
    use std::ptr::NonNull;

    /// A heap-allocated owner together with a handle that borrows from it, such as a
    /// prepared statement and the row stream reading from it.
    ///
    /// Replaces `owning_ref::OwningHandle` (RUSTSEC-2022-0040). That type keeps the
    /// owner in a `Box` while the handle borrows from it, which violates Rust's
    /// aliasing rules whenever the `Box` is moved. Here the owner is turned into a
    /// raw pointer with `Box::leak` for as long as the handle exists, so no
    /// `Box` claims unique access to it, and `Drop` frees the handle before the owner.
    ///
    /// Like the original, it dereferences through the handle (`H::Target`).
    pub(crate) struct OwningHandle<O, H> {
        handle: ManuallyDrop<H>,
        owner: NonNull<O>,
    }

    impl<O, H> OwningHandle<O, H> {
        /// Move `owner` to a stable heap address and build the handle from a pointer to it.
        ///
        /// `f` may turn the pointer into `&O` or `&mut O`; the owner stays alive, and
        /// at the same address, until the returned value is dropped. If `f` panics, the
        /// owner is leaked rather than freed.
        pub(crate) fn new_with_fn<F>(owner: Box<O>, f: F) -> Self
        where
            F: FnOnce(*const O) -> H,
        {
            // Give up the `Box` so nothing claims unique access while `H` borrows the
            // owner; `Drop` turns the pointer back into a `Box` to free it.
            let owner = NonNull::from(Box::leak(owner));
            let handle = f(owner.as_ptr());
            Self {
                handle: ManuallyDrop::new(handle),
                owner,
            }
        }
    }

    impl<O, H: Deref> Deref for OwningHandle<O, H> {
        type Target = H::Target;

        fn deref(&self) -> &Self::Target {
            &self.handle
        }
    }

    impl<O, H: DerefMut> DerefMut for OwningHandle<O, H> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.handle
        }
    }

    impl<O, H> Drop for OwningHandle<O, H> {
        fn drop(&mut self) {
            // SAFETY: the handle may borrow from the owner, so drop it first. `owner`
            // came from `Box::leak` in `new_with_fn` and is freed exactly once, here.
            unsafe {
                ManuallyDrop::drop(&mut self.handle);
                drop(Box::from_raw(self.owner.as_ptr()));
            }
        }
    }

    // SAFETY: the value owns both the owner and the handle outright, as
    // `(Box<O>, H)` would; `NonNull` only opts out of these impls by default.
    unsafe impl<O: Send, H: Send> Send for OwningHandle<O, H> {}
    unsafe impl<O: Sync, H: Sync> Sync for OwningHandle<O, H> {}

    #[cfg(test)]
    mod tests {
        use super::OwningHandle;
        use crate::utils::DummyBox;
        use std::cell::RefCell;
        use std::rc::Rc;

        /// Records its name in `log` when dropped.
        struct Noisy {
            name: &'static str,
            log: Rc<RefCell<Vec<&'static str>>>,
        }

        impl Drop for Noisy {
            fn drop(&mut self) {
                self.log.borrow_mut().push(self.name);
            }
        }

        #[test]
        fn owning_handle_borrows_owner_across_moves() {
            let owner = Box::new(vec![1, 2, 3]);
            let handle = OwningHandle::new_with_fn(owner, |v: *const Vec<i32>| unsafe {
                DummyBox((*v).iter())
            });
            // Moving the handle must not invalidate the borrow of the owner.
            let mut moved = handle;
            assert_eq!(moved.next(), Some(&1));
            let mut moved_again = moved;
            assert_eq!(moved_again.next(), Some(&2));
        }

        #[test]
        fn owning_handle_drops_handle_before_owner() {
            let log = Rc::new(RefCell::new(Vec::new()));
            let owner = Box::new(Noisy {
                name: "owner",
                log: log.clone(),
            });
            let handle_log = log.clone();
            let handle = OwningHandle::new_with_fn(owner, move |_| {
                DummyBox(Noisy {
                    name: "handle",
                    log: handle_log,
                })
            });
            drop(handle);
            assert_eq!(*log.borrow(), ["handle", "owner"]);
        }
    }
}

#[cfg(feature = "dst_arrow")]
pub fn decimal_to_i128(mut v: rust_decimal::Decimal, scale: u32) -> anyhow::Result<i128> {
    v.rescale(scale);

    let v_scale = v.scale();
    if v_scale != scale {
        return Err(anyhow::anyhow!(
            "decimal scale is not equal to expected scale, got: {} expected: {}",
            v_scale,
            scale
        ));
    }

    Ok(v.mantissa())
}

#[cfg(test)]
mod tests {
    use super::remove_query_params;
    use url::Url;

    fn strip(uri: &str, params: &[&str]) -> String {
        remove_query_params(&Url::parse(uri).unwrap(), params).to_string()
    }

    #[test]
    fn preserves_spaces_in_values() {
        assert_eq!(
            strip(
                "postgresql://u:p@host/db?options=-c%20statement_timeout%3D1s&cxprotocol=binary",
                &["cxprotocol"]
            ),
            "postgresql://u:p@host/db?options=-c%20statement_timeout%3D1s"
        );
    }

    #[test]
    fn does_not_reencode_remaining_params() {
        assert_eq!(
            strip("mysql://host/db?a=x+y&b=%2Fz&flag", &["nothing"]),
            "mysql://host/db?a=x+y&b=%2Fz&flag"
        );
    }

    #[test]
    fn removes_every_requested_param() {
        assert_eq!(
            strip(
                "postgresql://host/db?sslcert=a&keep=1&sslkey=b&sslrootcert=c",
                &["sslcert", "sslkey", "sslrootcert"]
            ),
            "postgresql://host/db?keep=1"
        );
    }

    #[test]
    fn drops_the_query_when_nothing_is_left() {
        assert_eq!(
            strip("postgresql://host/db?cxprotocol=binary", &["cxprotocol"]),
            "postgresql://host/db"
        );
    }

    #[test]
    fn handles_a_missing_query() {
        assert_eq!(
            strip("postgresql://host/db", &["cxprotocol"]),
            "postgresql://host/db"
        );
    }

    #[test]
    fn matches_percent_encoded_keys() {
        assert_eq!(
            strip(
                "postgresql://host/db?cx%70rotocol=binary&a=1",
                &["cxprotocol"]
            ),
            "postgresql://host/db?a=1"
        );
    }

    #[test]
    fn keeps_duplicate_keys_that_are_not_removed() {
        assert_eq!(
            strip("mysql://host/db?a=1&a=2&cxprotocol=text", &["cxprotocol"]),
            "mysql://host/db?a=1&a=2"
        );
    }
}
