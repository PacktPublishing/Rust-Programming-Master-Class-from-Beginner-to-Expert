   // -------------------------------------------
   // 			Project: Web Scrapping   
   // ------------------------------------------- 

   use std::sync::{mpsc,Arc,Mutex}; 
   use std::time::{Duration, Instant}; 
   use std::thread; 
   use ureq::{Agent, AgentBuilder}; 
   fn main() -> Result<(), ureq::Error>{  
    let webpages = vec![
        "https://gist.github.com/recluze/1d2989c7e345c8c3c542", 
        "https://gist.github.com/recluze/a98aa1804884ca3b3ad3",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/460157afc6a7492555bb",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/c9bc4130af995c36176d",
        "https://gist.github.com/recluze/1d2989c7e345c8c3c542",
        "https://gist.github.com/recluze/a98aa1804884ca3b3ad3",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/460157afc6a7492555bb",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/c9bc4130af995c36176d",
        "https://gist.github.com/recluze/1d2989c7e345c8c3c542",
        "https://gist.github.com/recluze/a98aa1804884ca3b3ad3",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/460157afc6a7492555bb",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/c9bc4130af995c36176d",
    ];  

    let agent = ureq::AgentBuilder::new().build();
    let now = Instant::now(); 
    
    for web_page in &webpages {
        let web_body = agent.get(web_page).call()?.into_string()?;
    }
    println!("Time taken wihtout Threads: {:.2?}", now.elapsed());

    let now = Instant::now(); 
    let agent = Arc::new(agent);
    let mut handles: Vec<thread::JoinHandle<Result<(), ureq::Error>>> = Vec::new(); 

    for web_page in webpages {
        let agent_thread = agent.clone(); 
        let t = thread::spawn(move || {
            let web_body = agent_thread
            .get(web_page)
            .call()?
            .into_string()?; 

            Ok(())
        });
        handles.push(t);
    } 

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Time taken using Threads: {:.2?}", now.elapsed());
    Ok(())

   }