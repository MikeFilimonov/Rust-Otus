type Transactions<'a> = Box<dyn Fn() -> &'a str>;

struct CashdeskShift<'a> {
    accepted: Vec<Transactions<'a>>,
    cancelled: Vec<Transactions<'a>>,
}

impl<'a> CashdeskShift<'a> {
    fn new() -> Self {
        Self {
            accepted: vec![],
            cancelled: vec![],
        }
    }

    fn sales_operation<F, C>(&mut self, accepted: F, cancelled: C)
    where
        F: Fn() -> &'a str + 'static,
        C: Fn() -> &'a str + 'static,
    {
        self.accepted.push(Box::new(accepted));
        self.cancelled.push(Box::new(cancelled));
    }

    fn accept_payment(&self) -> Vec<&str> {
        self.accepted
            .iter()
            .map(|transaction| transaction())
            .collect()
    }
    fn issue_refund(&self) -> Vec<&str> {
        self.cancelled
            .iter()
            .map(|transaction| transaction())
            .collect()
    }
}

fn inform_fts_about_sales() -> &'static str {
    "informed the Federal Tax Service of the sales transaction"
}

fn inform_fts_about_refund() -> &'static str {
    "informed the Federal Tax Service of the refund"
}

fn just_another_shift() -> CashdeskShift<'static> {
    let mut shift = CashdeskShift::new();
    shift.sales_operation(|| "paid with VISA", || "cancelled VISA payment");
    shift.sales_operation(inform_fts_about_sales, inform_fts_about_refund);
    shift
}

fn main() {
    let shift = just_another_shift();
    shift.accept_payment();
    shift.issue_refund();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successfully_paid() {
        let shift = just_another_shift();
        assert_eq!(
            vec![
                "paid with VISA",
                "informed the Federal Tax Service of the sales transaction"
            ],
            shift.accept_payment()
        );
    }

    #[test]
    fn applied_for_a_refund() {
        let shift = just_another_shift();
        assert_eq!(
            vec![
                "cancelled VISA payment",
                "informed the Federal Tax Service of the refund"
            ],
            shift.issue_refund()
        );
    }
}
