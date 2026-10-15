/*fn main()
{
    let mut flag = true; 
for a in 1..=1000{
    for b in a+1..1000 {
        for c in b+1..1000{ 
            if a*a + b*b == c*c && a + b+c == 1000 
            {
                println!("\n\n The required pathagorian triplet are ({}, {}, {}) \n\n", a,b,c); 
                flag = false; 
                break
            }
        }
        if flag == false {break}
    }
    if flag == false {break}
}

} 
*/

// alternate way using the return statement 
fn main()
{
for a in 1..=1000{
    for b in a+1..1000 {
        for c in b+1..1000{ 
            if a*a + b*b == c*c && a + b+c == 1000 
            {
                println!("\n\n The required pathagorian triplet are ({}, {}, {}) \n\n", a,b,c); 
                
                return; 
            }
        }
        
    }

}


}