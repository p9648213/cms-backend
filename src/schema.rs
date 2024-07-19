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
        identifier_id -> Nullable<Int4>,
        #[max_length = 255]
        api_key -> Nullable<Varchar>,
        #[max_length = 500]
        invoice_address -> Nullable<Varchar>,
        #[max_length = 255]
        invoice_email -> Nullable<Varchar>,
        #[max_length = 255]
        invoice_company_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    websites (id) {
        id -> Int4,
        agency_id -> Nullable<Int4>,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        domain -> Nullable<Varchar>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
    }
}

diesel::joinable!(websites -> agencies (agency_id));

diesel::allow_tables_to_appear_in_same_query!(agencies, websites,);
