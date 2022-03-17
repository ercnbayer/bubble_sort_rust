fn  bubble_sort(mut to_be_sorted: Vec<u32>)->Vec<u32>  
{
   for step in  0..to_be_sorted.len(){   
      let mut swapped=0;
      for i in  0..(to_be_sorted.len()-1-step)
   {

      if to_be_sorted[i]>to_be_sorted[i+1]
      {
         let temp=to_be_sorted[i];
         to_be_sorted[i]=to_be_sorted[i+1];
         to_be_sorted[i+1]=temp;
         swapped=1;  
      }
   }                  
      if swapped==0
   {
       break;
               }
               }
   to_be_sorted 
} 
fn main() {
    let mut to_be_sorted=vec![13,2,33,4,5,7,36,8,76,32,65,39];
   to_be_sorted=bubble_sort(to_be_sorted);
   for item in to_be_sorted
{ 
   println!("{}",item);
                  
}
    }

  
   
 



