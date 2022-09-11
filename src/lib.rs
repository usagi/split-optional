pub fn split2<'a>(input: &'a str, delimiter: &str) -> (Option<&'a str>, Option<&'a str>)
{
 match input.split_once(delimiter)
 {
  Some((a, b)) => (Some(a.trim()), Some(b.trim())),
  _ => (Some(input), None)
 }
}

pub fn split3<'a>(input: &'a str, delimiter: &str) -> (Option<&'a str>, Option<&'a str>, Option<&'a str>)
{
 let (a, tail) = split2(input, delimiter);
 let (b, c) = match tail
 {
  Some(tail) => split2(tail, delimiter),
  None => (None, None)
 };
 (a, b, c)
}

pub fn split4<'a>(input: &'a str, delimiter: &str) -> (Option<&'a str>, Option<&'a str>, Option<&'a str>, Option<&'a str>)
{
 let (a, b, tail) = split3(input, delimiter);
 let (c, d) = match tail
 {
  Some(tail) => split2(tail, delimiter),
  None => (None, None)
 };
 (a, b, c, d)
}

pub fn split5<'a>(input: &'a str, delimiter: &str)
 -> (Option<&'a str>, Option<&'a str>, Option<&'a str>, Option<&'a str>, Option<&'a str>)
{
 let (a, b, c, tail) = split4(input, delimiter);
 let (d, e) = match tail
 {
  Some(tail) => split2(tail, delimiter),
  None => (None, None)
 };
 (a, b, c, d, e)
}

pub fn split6<'a>(
 input: &'a str,
 delimiter: &str
) -> (
 Option<&'a str>,
 Option<&'a str>,
 Option<&'a str>,
 Option<&'a str>,
 Option<&'a str>,
 Option<&'a str>
)
{
 let (a, b, c, d, tail) = split5(input, delimiter);
 let (e, f) = match tail
 {
  Some(tail) => split2(tail, delimiter),
  None => (None, None)
 };
 (a, b, c, d, e, f)
}

pub fn split7<'a>(
 input: &'a str,
 delimiter: &str
) -> (
 Option<&'a str>,
 Option<&'a str>,
 Option<&'a str>,
 Option<&'a str>,
 Option<&'a str>,
 Option<&'a str>,
 Option<&'a str>
)
{
 let (a, b, c, d, e, tail) = split6(input, delimiter);
 let (f, g) = match tail
 {
  Some(tail) => split2(tail, delimiter),
  None => (None, None)
 };
 (a, b, c, d, e, f, g)
}

pub fn split8<'a>(
 input: &'a str,
 delimiter: &str
) -> (
 Option<&'a str>,
 Option<&'a str>,
 Option<&'a str>,
 Option<&'a str>,
 Option<&'a str>,
 Option<&'a str>,
 Option<&'a str>,
 Option<&'a str>
)
{
 let (a, b, c, d, e, f, tail) = split7(input, delimiter);
 let (g, h) = match tail
 {
  Some(tail) => split2(tail, delimiter),
  None => (None, None)
 };
 (a, b, c, d, e, f, g, h)
}
