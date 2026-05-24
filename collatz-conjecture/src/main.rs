use collatz_conjecture::collatz;

fn main() {
    let mut colls = Vec::new();

    for i in 2..=10000000 {
        let result = collatz(i);

        if result != None {
            colls.push(result.unwrap());
        }
        if i % 100000 == 0 {
            println!("i is {i}");
        }
    }
    colls.sort();

    println!("Colls is sorted ");

    let mut j: usize = 0;
    let mut j_keep = j;
    let mut old_count =0 ;

    // println! (" Lenght of colls is {}", colls.len() );
    loop {
        //println! ("J is {}, colls is {} long ", j, colls.len());
        if j >= colls.len() - 1 {
            break;
        }
        if colls[j] == colls[j + 1] {
            colls.remove(j + 1);
        } else {
            j += 1;
        }

        if j != j_keep {
            let new_count = colls.len() ;
            if old_count > new_count {
                println!( "Removed {} entries", old_count - new_count)
            }
            println!("j is {j}, colls has {} elements ", new_count);
            j_keep = j;
            old_count = new_count ;
        }
    }

    println!(" Colls is {:?}", colls);
}
