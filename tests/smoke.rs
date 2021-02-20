//! Simple native_fn testing.

use alchem::collapsed::CollapsedInfo;
use alchem::value::{Globals, MorphStatus, NativeInfo, NoCustom, Type, Value, CustomType};
use alchem::vm::Runner;
use alchem_macros::{native_fn, native_tfn};

type Val = Value<NoCustom>;
type Info = NativeInfo<NoCustom>;
type CoInfo = CollapsedInfo<NoCustom>;
type Run = Runner<NoCustom>;
type Tp = Type<NoCustom>;
type Gl = Globals<NoCustom>;
type Status = MorphStatus<NoCustom>;

#[native_tfn]
async fn ntvt_number(_args: Vec<Tp>, _globals: &Option<Gl>) -> Status {
  MorphStatus::NativeCompleted(Info::new(), Type::Number)
}

#[native_fn]
async fn ntv_recall(_vals: Vec<Val>, _info: Option<CoInfo>, _runner: &mut Option<Run>) -> Val {
  Val::String("hello".into())
}

#[native_tfn]
async fn ntvt_eval<C: CustomType + 'static>(_args: Vec<Type<C>>, _globals: &Globals<C>) -> MorphStatus<C> {
  let info = NativeInfo::new();
  MorphStatus::NativeCompleted(info, Type::String(None))
}

#[native_fn]
async fn ntv_eval<C: CustomType + 'static>(
  _vals: Vec<Value<C>>, _info: Option<CollapsedInfo<C>>, _runner: &mut Option<Runner<C>>
) -> Value<C> {
  Value::String("fake".into())
}

#[tokio::test]
async fn works_tfn() {
  assert!(ntvt_number(Vec::new(), &None).await.is_known());
}

#[tokio::test]
async fn works_fn() {
  assert_eq!(ntv_recall(Vec::new(), None, &mut None).await.as_str(), "hello");
}

#[tokio::test]
async fn works_tfn_c() {
  assert!(ntvt_eval(Vec::new(), &Gl::new()).await.is_known());
}

#[tokio::test]
async fn works_fn_c() {
  assert_eq!(ntv_eval(vec![Val::Int(1)], None, &mut None).await.as_str(), "fake");
}
