use crate::CXFederatedPlan;
use connectorx::fed_rewriter::Plan;
use libc::c_char;
use std::convert::Into;
use std::ffi::CString;

impl Into<CXFederatedPlan> for Plan {
    fn into(self) -> CXFederatedPlan {
        CXFederatedPlan {
            db_name: CString::new(self.db_name.as_str())
                .expect("new CString error")
                .into_raw() as *const c_char,
            db_alias: CString::new(self.db_alias.as_str())
                .expect("new CString error")
                .into_raw() as *const c_char,
            sql: CString::new(self.sql.as_str())
                .expect("new CString error")
                .into_raw() as *const c_char,
            cardinality: self.cardinality,
        }
    }
}
