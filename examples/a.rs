fn main()
{
 const SOME_INPUT: &str = "aaa,bbb,ccc";

 // (Some("aaa"), Some("bbb,ccc"))
 let result = split_optional::split2(SOME_INPUT, ",");
 println!("{result:?}");

 // (Some("aaa"), Some("bbb"), Some("ccc"))
 let result = split_optional::split3(SOME_INPUT, ",");
 println!("{result:?}");

 // (Some("aaa"), Some("bbb"), Some("ccc"), None)
 let result = split_optional::split4(SOME_INPUT, ",");
 println!("{result:?}");

 // (Some("aaa"), Some("bbb"), Some("ccc"), None, None)
 let result = split_optional::split5(SOME_INPUT, ",");
 println!("{result:?}");
}
