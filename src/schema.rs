// @generated automatically by Diesel CLI.

diesel::table! {
    agencies (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 20]
        phone_number -> Nullable<Varchar>,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        #[max_length = 20]
        tax_code -> Nullable<Varchar>,
        #[max_length = 500]
        invoice_address -> Nullable<Varchar>,
        #[max_length = 255]
        invoice_email -> Nullable<Varchar>,
        #[max_length = 255]
        invoice_company_name -> Nullable<Varchar>,
    }
}
