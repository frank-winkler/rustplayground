// pub fn contains<T: 'a>(array: &'a [T], target: T) -> bool where &'a T: PartialEq<T> {
//     for &i in array {
//         if *i == target {
//             return true;
//         }
//     }
//     false
// }