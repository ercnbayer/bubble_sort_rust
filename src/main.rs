fn  bubble_sort(  mut vec: Vec<u32>)->Vec<u32>
{
    
    for step in  0..vec.len(){
      let mut swapped=0;
   for i in  0..(vec.len()-1-step)
   {

if vec[i]>vec[i+1]
{
let temp=vec[i];
vec[i]=vec[i+1];
vec[i+1]=temp;
swapped=1;
}




   }
   if swapped==0
   {
       break;
}
   }
   
  vec 
}

fn main() {
    let mut vec=vec![13,2,33,4,5,7,36,8,76,32,65,39];
vec=bubble_sort(vec);
for item in vec
{ println!("{}",item);
 }
    }
