use crate::CXFederatedPlan;
use connectorx::fed_rewriter::Plan;
use libc::c_char;
use std::ffi::CString;

impl From<Plan> for CXFederatedPlan {
    fn from(plan: Plan) -> Self {
        CXFederatedPlan {
            db_name: CString::new(plan.db_name.as_str())
                .expect("new CString error")
                .into_raw() as *const c_char,
            db_alias: CString::new(plan.db_alias.as_str())
                .expect("new CString error")
                .into_raw() as *const c_char,
            sql: CString::new(plan.sql.as_str())
                .expect("new CString error")
                .into_raw() as *const c_char,
            cardinality: plan.cardinality,
        }
    }
}
