/// Generates a `user_data` context containing a reference to a single or several Java callbacks
macro_rules! gen_ctx {
    ($env:ident, $cb:ident) => {
        {
            let ctx = $env.new_global_ref($cb).unwrap().into_raw_ptr();
            $env.delete_local_ref($cb).unwrap();
            ctx
        }
    };

    ($env:ident, $cb0:ident, $($cb_rest:ident),+ ) => {
        {
            let ctx = [
                Some($env.new_global_ref($cb0).unwrap()),
                $(
                    Some($env.new_global_ref($cb_rest).unwrap()),
                )+
            ];
            let ctx = Box::into_raw(Box::new(ctx)) as *mut c_void;
            $env.delete_local_ref($cb0).unwrap();
            $(
                $env.delete_local_ref($cb_rest).unwrap();
            )+
            ctx
        }
    }
}
