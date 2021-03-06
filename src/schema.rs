table! {
    cart_items_session (id) {
        id -> Uuid,
        session_id -> Int4,
        product_id -> Int4,
        quantity -> Int4,
        store_id -> Int4,
        comment -> Varchar,
        selected -> Bool,
        pre_order -> Bool,
        pre_order_days -> Int4,
        coupon_id -> Nullable<Int4>,
        delivery_method_id -> Nullable<Jsonb>,
        currency_type -> Varchar,
        user_country_code -> Nullable<Text>,
    }
}

table! {
    cart_items_user (id) {
        user_id -> Int4,
        product_id -> Int4,
        quantity -> Int4,
        selected -> Bool,
        store_id -> Int4,
        comment -> Nullable<Varchar>,
        id -> Uuid,
        pre_order -> Bool,
        pre_order_days -> Int4,
        coupon_id -> Nullable<Int4>,
        delivery_method_id -> Nullable<Jsonb>,
        currency_type -> Varchar,
        user_country_code -> Nullable<Text>,
    }
}

table! {
    order_diffs (id) {
        id -> Uuid,
        parent -> Uuid,
        committer -> Int4,
        committed_at -> Timestamptz,
        state -> Varchar,
        comment -> Nullable<Varchar>,
        committer_role -> Varchar,
    }
}

table! {
    orders (id) {
        id -> Uuid,
        slug -> Int4,
        store -> Int4,
        customer -> Int4,
        product -> Int4,
        price -> Float8,
        quantity -> Int4,
        receiver_name -> Varchar,
        administrative_area_level_1 -> Nullable<Varchar>,
        administrative_area_level_2 -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        locality -> Nullable<Varchar>,
        political -> Nullable<Varchar>,
        postal_code -> Nullable<Varchar>,
        route -> Nullable<Varchar>,
        street_number -> Nullable<Varchar>,
        address -> Nullable<Varchar>,
        place_id -> Nullable<Varchar>,
        track_id -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        state -> Varchar,
        payment_status -> Bool,
        delivery_company -> Nullable<Varchar>,
        created_from -> Uuid,
        conversion_id -> Uuid,
        receiver_phone -> Nullable<Varchar>,
        currency -> Varchar,
        pre_order -> Bool,
        pre_order_days -> Int4,
        coupon_id -> Nullable<Int4>,
        product_discount -> Nullable<Float8>,
        coupon_percent -> Nullable<Int4>,
        coupon_discount -> Nullable<Float8>,
        total_amount -> Float8,
        receiver_email -> Nullable<Varchar>,
        company_package_id -> Nullable<Int4>,
        delivery_price -> Float8,
        shipping_id -> Nullable<Int4>,
        uuid -> Uuid,
        product_cashback -> Nullable<Float8>,
        currency_type -> Varchar,
    }
}

table! {
    roles (id) {
        id -> Uuid,
        user_id -> Int4,
        name -> Varchar,
        data -> Jsonb,
    }
}

joinable!(order_diffs -> orders (parent));

allow_tables_to_appear_in_same_query!(
    cart_items_session,
    cart_items_user,
    order_diffs,
    orders,
    roles,
);
