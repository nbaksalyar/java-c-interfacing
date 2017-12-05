/// Generates a `user_data` context containing a reference to a single or several Java callbacks
macro_rules! gen_ctx {
    ($env:ident, $cb:ident) => {
        {
            let ctx = $env.new_global_ref($cb).unwrap().detach().unwrap();
            $env.delete_local_ref($cb).unwrap();
            ctx.into_inner() as *mut c_void
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

/// Generates primitive type converters
macro_rules! gen_primitive_type_converter {
    ($native_type:ty, $java_type:ty) => {
        impl FromJava<$java_type> for $native_type {
            fn from_java(_env: &JNIEnv, input: $java_type) -> Self {
                input as Self
            }
        }

        impl<'a> ToJava<'a, $java_type> for $native_type {
            fn to_java(&self, _env: &JNIEnv) -> $java_type {
                *self as $java_type
            }
        }
    }
}

macro_rules! gen_byte_array_converter {
    ($arr_type:ty, $size:expr) => {
        impl<'a> FromJava<JObject<'a>> for [$arr_type; $size] {
            fn from_java(env: &JNIEnv, input: JObject) -> Self {
                let input = input.into_inner() as jni::sys::jbyteArray;
                let mut output = [0; $size];
                env.get_byte_array_region(input, 0, &mut output).unwrap();

                unsafe { mem::transmute(output) }
            }
        }

        impl<'a> ToJava<'a, JObject<'a>> for [$arr_type; $size] {
            fn to_java(&self, env: &'a JNIEnv) -> JObject<'a> {
                let output = env.new_byte_array(self.len() as jni::sys::jsize).unwrap();
                env.set_byte_array_region(output, 0, unsafe {
                    slice::from_raw_parts(self.as_ptr() as *const i8, self.len())
                }).unwrap();
                JObject::from(output as jni::sys::jobject)
            }
        }
    }
}
