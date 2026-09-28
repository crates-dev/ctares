mod r#const;

pub use r#const::*;

use lombok_macros::*;
use std::{f64::consts::PI, fmt::Debug};

#[derive(Clone, Data, Debug, DisplayDebugFormat)]
struct LombokTest<'a, T: Clone + Debug> {
    #[get(pub(crate))]
    #[set(pub(crate))]
    list: Vec<String>,
    #[get(pub(crate))]
    opt_value: Option<&'a T>,
    #[get(pub(crate))]
    result_value: Result<&'a T, &'static str>,
    #[get_mut(pub(crate))]
    #[set(private)]
    name: String,
    #[get_mut(pub(crate))]
    #[set(private)]
    user: User,
}

#[derive(Clone, CustomDebug, Getter, New, Setter)]
struct User {
    #[set(type(AsRef<str>))]
    name: String,
    #[debug(skip)]
    _password: String,
    #[new(skip)]
    email: Option<String>,
}

#[derive(Clone, Data, Debug)]
struct TupleStruct(
    String,
    i32,
    bool,
);

#[derive(Clone, Data, Debug)]
struct TraitTestStruct {
    #[set(type(AsRef<str>))]
    name: String,
    #[get(type(clone))]
    #[set(type(Into<i32>))]
    value: i32,
    #[set(type(AsRef<[u8]>))]
    data: Vec<u8>,
    #[set(type(Into<Vec<String>>))]
    items: Vec<String>,
}

#[derive(Clone, Data, Debug)]
struct TupleWithResult(
    #[get(type(clone))] String,
    Result<i32, &'static str>,
);

#[derive(CustomDebug)]
enum Response {
    Success {
        data: String,
    },
    Error {
        message: String,
        #[debug(skip)]
        _internal_code: u32,
    },
}

#[derive(New)]
struct Person {
    name: String,
    _age: u32,
}

#[derive(New)]
#[new]
struct PublicPerson {
    _name: String,
    _age: u32,
}

#[derive(New)]
#[new(pub(crate))]
struct CratePerson {
    _name: String,
    _age: u32,
}

#[derive(New)]
#[new(private)]
struct PrivatePerson {
    _name: String,
    _age: u32,
}

#[derive(Data, New)]
#[new(private)]
struct Product {
    id: u64,
    name: String,
    _price: f64,
    #[new(skip)]
    _description: String,
}

#[derive(New)]
struct TuplePoint(f64, #[new(skip)] f64, f64);

#[derive(Clone, Data, Debug)]
struct NestedStruct {
    name: String,
    _value: i32,
}

#[derive(Clone, Data, Debug)]
struct ComplexNestedStruct {
    nested: NestedStruct,
    nested_list: Vec<NestedStruct>,
    metadata: std::collections::HashMap<String, String>,
}

#[derive(CustomDebug)]
enum ComplexEnum {
    Simple,
    Tuple(String, i32),
    Struct {
        field1: String,
        #[debug(skip)]
        _secret: String,
        value: f64,
    },
}

#[derive(New)]
struct GenericStruct<T: Default + Clone> {
    #[new(skip)]
    data: T,
    value: i32,
}

#[derive(Clone, Data, Debug)]
struct LifetimesTest<'a, 'b> {
    name: &'a str,
    description: &'b str,
}

#[derive(Clone, Data, Debug)]
struct EdgeCaseTest {
    empty_string: String,
    #[get(type(clone))]
    empty_vec: Vec<i32>,
    #[get(type(clone))]
    zero_value: i32,
    #[get(type(clone))]
    bool_false: bool,
    option_none: Option<String>,
}

#[derive(Clone, Data, Debug)]
struct CopyTest {
    #[get(skip)]
    _value: i32,
    #[get(pub(crate), type(copy))]
    flag: bool,
    #[get(private, type(copy))]
    count: u64,
}

#[derive(Data)]
struct UnitGetSet {
    flag: bool,
}

#[derive(New)]
struct AllSkipped {
    #[new(skip)]
    skipped1: String,
    #[new(skip)]
    skipped2: i32,
}

#[derive(Clone, Data, Debug)]
struct MultiAttributes {
    #[get(type(clone))]
    #[set(pub(crate), type(Into<Vec<String>>))]
    complex_field: Vec<String>,
}

/// Runs the lombok-macros integration sandbox.
///
/// Exercises every derive macro and accessor shape against runtime
/// assertions so a regression in code generation fails the build.
fn main() {
    let mut data: LombokTest<usize> = LombokTest {
        list: Vec::new(),
        opt_value: None,
        result_value: Err(SANDBOX_ERR_TEXT),
        name: SANDBOX_TEST_NAME.to_string(),
        user: User {
            name: SANDBOX_USER_NAME.to_string(),
            _password: SANDBOX_PASSWORD.to_string(),
            email: Some(SANDBOX_EMAIL.to_string()),
        },
    };
    let user: &mut User = data.get_mut_user();
    user.set_name("Bob");
    assert_eq!(data.get_user().get_name(), "Bob");
    let list: Vec<String> = vec![SANDBOX_HELLO.to_string(), SANDBOX_WORLD.to_string()];
    data.set_list(list.clone());
    assert_eq!(*data.get_list(), list);
    let opt_value: &Option<&usize> = data.try_get_opt_value();
    assert_eq!(*opt_value, None);
    data.set_opt_value(Some(&42));
    let try_opt_value: &Option<&usize> = data.try_get_opt_value();
    assert_eq!(try_opt_value, &Some(&42));
    let unwrap_value: &usize = data.get_opt_value();
    assert_eq!(unwrap_value, &42);
    let result_value: &Result<&usize, &str> = data.try_get_result_value();
    assert_eq!(*result_value, Err(SANDBOX_ERR_TEXT));
    data.set_result_value(Ok(&100));
    let try_result_value: &Result<&usize, &str> = data.try_get_result_value();
    assert_eq!(try_result_value, &Ok(&100));
    let unwrap_result: &usize = data.get_result_value();
    assert_eq!(unwrap_result, &100);
    let name_mut: &mut String = data.get_mut_name();
    *name_mut = SANDBOX_UPDATED.to_string();
    assert!(!data.to_string().is_empty());
    let mut tuple_data: TupleStruct = TupleStruct(SANDBOX_HELLO.to_string(), 42, true);
    let field0: &String = tuple_data.get_0();
    assert_eq!(field0, SANDBOX_HELLO);
    tuple_data.set_1(100);
    let field2: &bool = tuple_data.get_2();
    assert!(*field2);
    tuple_data.set_2(false);
    let mut tuple_result: TupleWithResult = TupleWithResult(SANDBOX_TEST_NAME.to_string(), Err(SANDBOX_ERR_TEXT));
    let try_result: String = tuple_result.get_0();
    assert_eq!(try_result, String::from(SANDBOX_TEST_NAME));
    let try_result: &Result<i32, &str> = tuple_result.try_get_1();
    assert_eq!(*try_result, Err(SANDBOX_ERR_TEXT));
    tuple_result.1 = Ok(42);
    let unwrap_result: i32 = tuple_result.get_1();
    assert_eq!(unwrap_result, 42);
    let user: User = User {
        name: SANDBOX_USER_NAME.to_string(),
        _password: SANDBOX_PASSWORD.to_string(),
        email: Some(SANDBOX_EMAIL.to_string()),
    };
    assert_eq!(user.get_name(), SANDBOX_USER_NAME);
    assert_eq!(user.get_email(), SANDBOX_EMAIL.to_string());
    let user_debug: String = format!("{user:?}");
    assert!(user_debug.contains(SANDBOX_USER_NAME));
    assert!(user_debug.contains(SANDBOX_EMAIL));
    assert!(!user_debug.contains(SANDBOX_PASSWORD));
    let success: Response = Response::Success {
        data: SANDBOX_SUCCESS_TEXT.to_string(),
    };
    let success_debug: String = format!("{success:?}");
    assert!(success_debug.contains(SANDBOX_SUCCESS_TEXT));
    let error: Response = Response::Error {
        message: SANDBOX_FAILURE_TEXT.to_string(),
        _internal_code: 500,
    };
    let error_debug: String = format!("{error:?}");
    assert!(error_debug.contains(SANDBOX_FAILURE_TEXT));
    assert!(!error_debug.contains("500"));
    let person: Person = Person::new(SANDBOX_USER_NAME.to_string(), 30);
    assert_eq!(person.name, SANDBOX_USER_NAME);
    let user: User = User::new(SANDBOX_USERNAME.to_string(), SANDBOX_USERNAME.to_string());
    assert_eq!(user.email, None);
    let product: Product = Product::new(1, SANDBOX_PRODUCT_NAME.to_string(), 999.99);
    assert_eq!(*product.get_id(), 1);
    assert_eq!(product.get_name(), SANDBOX_PRODUCT_NAME);
    let tuple_point: TuplePoint = TuplePoint::new(10.5, 30.5);
    assert_eq!(tuple_point.0, 10.5);
    assert_eq!(tuple_point.1, 0.0);
    assert_eq!(tuple_point.2, 30.5);
    let public_person: PublicPerson = PublicPerson::new(SANDBOX_USER_NAME.to_string(), 25);
    assert_eq!(public_person._name, SANDBOX_USER_NAME);
    assert_eq!(public_person._age, 25);
    let crate_person: CratePerson = CratePerson::new("Bob".to_string(), 35);
    assert_eq!(crate_person._name, "Bob");
    assert_eq!(crate_person._age, 35);
    let private_person: PrivatePerson = PrivatePerson::new(SANDBOX_PRIVATE_NAME.to_string(), 45);
    assert_eq!(private_person._name, SANDBOX_PRIVATE_NAME);
    assert_eq!(private_person._age, 45);
    let mut trait_test: TraitTestStruct = TraitTestStruct {
        name: SANDBOX_TEST_NAME.to_string(),
        value: 42,
        data: vec![1, 2, 3],
        items: vec![SANDBOX_ITEM_ONE.to_string(), SANDBOX_ITEM_TWO.to_string()],
    };
    trait_test.set_name(SANDBOX_RENAMED);
    trait_test.set_value(100);
    trait_test.set_data([4, 5, 6, 7]);
    let new_items: Vec<String> = vec![SANDBOX_NEW_ITEM_ONE.to_string(), SANDBOX_NEW_ITEM_TWO.to_string()];
    trait_test.set_items(new_items);
    assert_eq!(*trait_test.get_name(), SANDBOX_RENAMED);
    assert_eq!(trait_test.get_value(), 100);
    assert_eq!(*trait_test.get_data(), vec![4, 5, 6, 7]);
    assert_eq!(
        *trait_test.get_items(),
        vec![SANDBOX_NEW_ITEM_ONE.to_string(), SANDBOX_NEW_ITEM_TWO.to_string()]
    );
    let nested: NestedStruct = NestedStruct {
        name: SANDBOX_NESTED_NAME.to_string(),
        _value: 42,
    };
    let mut complex: ComplexNestedStruct = ComplexNestedStruct {
        nested: nested.clone(),
        nested_list: vec![nested],
        metadata: std::collections::HashMap::new(),
    };
    complex.set_metadata({
        let mut map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        map.insert(SANDBOX_MAP_KEY.to_string(), SANDBOX_MAP_VALUE.to_string());
        map
    });
    assert_eq!(complex.get_nested().get_name(), SANDBOX_NESTED_NAME);
    assert_eq!(complex.get_nested_list().len(), 1);
    assert_eq!(complex.get_metadata().get("key").unwrap(), "value");
    let simple: ComplexEnum = ComplexEnum::Simple;
    let tuple: ComplexEnum = ComplexEnum::Tuple(SANDBOX_TEST_NAME.to_string(), 123);
    let struct_variant: ComplexEnum = ComplexEnum::Struct {
        field1: SANDBOX_VISIBLE.to_string(),
        _secret: SANDBOX_HIDDEN.to_string(),
        value: PI,
    };
    let simple_debug: String = format!("{simple:?}");
    let tuple_debug: String = format!("{tuple:?}");
    let struct_debug: String = format!("{struct_variant:?}");
    assert!(simple_debug.contains("Simple"));
    assert!(tuple_debug.contains(SANDBOX_TEST_NAME));
    assert!(tuple_debug.contains("123"));
    assert!(struct_debug.contains(SANDBOX_VISIBLE));
    assert!(!struct_debug.contains(SANDBOX_HIDDEN));
    assert!(struct_debug.contains("3.14"));
    let generic_i32: GenericStruct<i32> = GenericStruct::<i32> {
        data: 0,
        value: 100,
    };
    let generic_string: GenericStruct<String> = GenericStruct::<String>::new(200);
    assert_eq!(generic_i32.value, 100);
    assert_eq!(generic_i32.data, 0);
    assert_eq!(generic_string.value, 200);
    assert_eq!(generic_string.data, "");
    let name: &str = SANDBOX_LIFETIME_NAME;
    let description: &str = SANDBOX_LIFETIME_DESCRIPTION;
    let lifetimes_test: LifetimesTest<'_, '_> = LifetimesTest { name, description };
    assert_eq!(*lifetimes_test.get_name(), SANDBOX_LIFETIME_NAME);
    assert_eq!(*lifetimes_test.get_description(), SANDBOX_LIFETIME_DESCRIPTION);
    let edge_case: EdgeCaseTest = EdgeCaseTest {
        empty_string: String::new(),
        empty_vec: Vec::new(),
        zero_value: 0,
        bool_false: false,
        option_none: None,
    };
    assert_eq!(edge_case.get_empty_string(), "");
    assert!(edge_case.get_empty_vec().is_empty());
    assert_eq!(edge_case.get_zero_value(), 0);
    assert!(!edge_case.get_bool_false());
    assert!(edge_case.try_get_option_none().is_none());
    let unit_get: UnitGetSet = UnitGetSet { flag: true };
    let flag_ref: &bool = unit_get.get_flag();
    assert!(*flag_ref);
    let constructed: AllSkipped = AllSkipped::new();
    assert_eq!(constructed.skipped1, "");
    assert_eq!(constructed.skipped2, 0);
    let multi: MultiAttributes = MultiAttributes {
        complex_field: vec![SANDBOX_TEST_NAME.to_string()],
    };
    let cloned_field: Vec<String> = multi.get_complex_field();
    assert_eq!(cloned_field, vec![SANDBOX_TEST_NAME.to_string()]);
    let mut mutated: MultiAttributes = multi;
    let new_vec: Vec<String> = vec![SANDBOX_NEW_VALUE.to_string(), SANDBOX_VALUES.to_string()];
    mutated.set_complex_field(new_vec.clone());
    let updated: Vec<String> = mutated.get_complex_field();
    assert_eq!(updated, new_vec);
    let copy_test: CopyTest = CopyTest {
        _value: 42,
        flag: true,
        count: 1000,
    };
    let copied_flag: bool = copy_test.get_flag();
    let copied_count: u64 = copy_test.get_count();
    assert!(copied_flag);
    assert_eq!(copied_count, 1000);
    let mut value: u8 = 7;
    let mut generic_ptr: GenericPtr<u8> = GenericPtr {
        ptr: &mut value as *mut u8,
    };
    assert_eq!(unsafe { **generic_ptr.get_ptr() }, 7);
    generic_ptr.set_ptr(std::ptr::null_mut());
    assert!(generic_ptr.get_ptr().is_null());
    let mut callback: Box<dyn FnMut()> = Box::new(|| {});
    let callback_ptr: *mut dyn FnMut() = &mut *callback;
    let mut dst_ptr: DstPtr = DstPtr { ptr: callback_ptr };
    assert!(!dst_ptr.get_ptr().is_null());
    assert!(!dst_ptr.get_mut_ptr().is_null());
    let mut callback_two: Box<dyn FnMut()> = Box::new(|| {});
    dst_ptr.set_ptr(&mut *callback_two as *mut dyn FnMut());
    assert!(!dst_ptr.get_ptr().is_null());
    let mut const_callback: Box<dyn FnMut()> = Box::new(|| {});
    let mut const_ptr: ConstPtr = ConstPtr {
        cptr: &mut *const_callback as *mut dyn FnMut(),
    };
    assert!(!const_ptr.get_cptr().is_null());
    let const_debug: String = format!("{const_ptr:?}");
    assert!(const_debug.contains("cptr"));
    const_ptr.set_cptr(&*callback as *const dyn FnMut());
    assert!(!const_ptr.get_cptr().is_null());
    let mut opt_ptr: OptPtr = OptPtr { opt: None };
    assert!(opt_ptr.try_get_opt().is_none());
    opt_ptr.set_opt(Some(&mut value as *mut u8));
    assert_eq!(unsafe { *opt_ptr.get_opt() }, 7);
    assert!(opt_ptr.try_get_opt().is_some());
    let mut opt_dst_ptr: OptDstPtr = OptDstPtr { opt: None };
    assert!(opt_dst_ptr.try_get_opt().is_none());
    opt_dst_ptr.set_opt(Some(&mut *callback as *mut dyn FnMut()));
    assert!(!opt_dst_ptr.get_opt().is_null());
    let copy_ptr: CopyPtr = CopyPtr {
        ptr: &mut value as *mut u8,
    };
    let copied_ptr: *mut u8 = copy_ptr.get_ptr();
    assert_eq!(unsafe { *copied_ptr }, 7);
    let mut tuple_ptr: TuplePtr = TuplePtr(&mut *callback_two as *mut dyn FnMut(), 3);
    assert!(!tuple_ptr.get_0().is_null());
    assert_eq!(*tuple_ptr.get_1(), 3);
    let mut callback_three: Box<dyn FnMut()> = Box::new(|| {});
    tuple_ptr.set_0(&mut *callback_three as *mut dyn FnMut());
    assert!(!tuple_ptr.get_0().is_null());
    let dst_debug: String = format!("{dst_ptr:?}");
    assert!(dst_debug.contains("ptr"));
}

#[derive(Clone, Data, Debug)]
struct GenericPtr<T> {
    ptr: *mut T,
}

#[derive(Clone, Data, Debug)]
struct DstPtr {
    ptr: *mut dyn FnMut(),
}

#[derive(Clone, Data, Debug)]
struct ConstPtr {
    cptr: *const dyn FnMut(),
}

#[derive(Clone, Data, Debug)]
struct OptPtr {
    opt: Option<*mut u8>,
}

#[derive(Clone, Data, Debug)]
struct OptDstPtr {
    opt: Option<*mut dyn FnMut()>,
}

#[derive(Clone, Data, Debug)]
struct CopyPtr {
    #[get(type(copy))]
    ptr: *mut u8,
}

#[derive(Clone, Data, Debug)]
struct TuplePtr(*mut dyn FnMut(), i32);
