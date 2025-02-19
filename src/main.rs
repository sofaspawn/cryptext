use std::collections::HashSet;
use std::{io::stdout, vec};

use rand::Rng;
use rand::distr::{Distribution, Uniform};

use crossterm::{
    cursor::{self, MoveTo}, execute, style::Print, terminal
};

pub struct AlphanumericWithSpace;

impl Distribution<char> for AlphanumericWithSpace {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        let chars = "!@#$%^&*()_+-=[]{}\\|\"\':;/?>.<,abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ";
        let chars_len = chars.len();
        let index_distribution_result: Result<Uniform<usize>, _> = Uniform::new(0, chars_len);
        let index_distribution = match index_distribution_result {
            Ok(dist) => dist,
            Err(err) => panic!("Failed to create Uniform distribution: {:?}", err),
        };
        // Sample an index from the distribution
        let index = index_distribution.sample(rng);
        // Return the character at the randomly selected index
        chars.chars().nth(index).unwrap()
    }
}

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let start = std::time::Instant::now();

    execute!(stdout(), cursor::Hide).unwrap();
    execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();

    let (width, height) = terminal::size().unwrap();

    //let text = String::from("Random Cypher, Text.");
    let text = args[1].clone();

    let og_vec = text.bytes().collect::<Vec<u8>>();
    let mut s_vec = vec![0;og_vec.len()];

    let mut idx_done:Vec<usize> = vec![];
    let mut end = false;

    while !end{
        //let n_vec = (0..=og_vec.len()).map(|_| random::<char>()).collect::<String>().bytes().collect::<Vec<u8>>();
        let n_vec = rand::rng()
        .sample_iter(&AlphanumericWithSpace)
        .take(og_vec.len())
        .map(char::from)
        .collect::<String>()
        .bytes()
        .collect::<Vec<u8>>();

        assert_eq!(n_vec.len(), og_vec.len());

        for i in 0..n_vec.len(){
            if og_vec[i]==n_vec[i]{
                s_vec[i] = og_vec[i];
                idx_done.push(i);
            } else {
                if idx_done.contains(&i){
                    continue;
                }
                s_vec[i] = n_vec[i];
            }
            let set:HashSet<_> = idx_done.iter().cloned().collect();
            let check:Vec<_> = set.into_iter().collect();
            if check.len()==s_vec.len(){
                end = true;
                break;
            }
        }

        let rend_text = String::from_utf8(s_vec.clone()).unwrap();

        execute!(
            stdout(),
            MoveTo((width-og_vec.len() as u16)/2, height/2),
            Print(format!("{rend_text}\n"))
        ).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(90));
    }

    let time_elapsed = (std::time::Instant::now() - start).as_secs_f32();
    let elapsed_time_format = format!("Time Elapsed: {time_elapsed} seconds");

    execute!(
        stdout(),
        MoveTo((width-elapsed_time_format.len() as u16)/2, height-4),
        Print(elapsed_time_format)
    ).unwrap();

    execute!(
        stdout(),
        MoveTo(0, height-1),
    ).unwrap();

    execute!(stdout(), cursor::Show).unwrap();
    Ok(())
}
