#![allow(warnings)]
enum Resources{
    Library,
    WeeklyDigest,
    Kindle,
}
trait Readable{
    fn read(&self);
}
struct Publication{
    title: String,
    author: String,
    publication_year: u16,
    avail_through: Resources,
}

impl Publication {
    // return immutable reference
    fn get_title(&self) -> &String{
        &self.title
    }

    fn get_resource(&self) -> &Resources{
        &self.avail_through
    }
}
struct  Book{
    publication: Publication,
    publisher: String,
    num_pages: u32,
}
impl Book{
    fn get_publisher(&self) -> &String{
        &self.publisher
    }

    fn new(publication: Publication, publisher: String, num_pages: u32) -> Self{
        Self{
            publication,
            publisher,
            num_pages,
        }
    }
}


struct Novel{
    book: Book,
    genre: String,
}
impl Novel{
    fn new(book: Book, genre: String) -> Self{
        Self{
            book,
            genre,
        }
    }
}

impl Readable for Novel{
    fn read(&self) {
        println!("Reading Novel: {}",self.book.publication.title);
    }
}
struct Magazine{
    publication: Publication,
    issue: String
}
impl Magazine{
    fn new(publication: Publication, issue: String) -> Self{
        Self{
            publication,
            issue,
        }
    }
}
impl Readable for Magazine{
    fn read(&self){
        println!("Reading Magazine: {}",self.publication.title);
    }
}
struct ResearchPaper{
    publication: Publication,
    topic: String,
}
impl ResearchPaper{
    fn new(publication: Publication,topic: String) -> Self{
        Self{
            publication,
            topic,
        }
    }
}

fn main () {
    let book = Book::new(Publication{
       title: "Power of now".to_string(),
       author: "Echart tolle".to_string(),
        publication_year: 2019,
        avail_through: Resources::Library,
    },"pubs 1".to_string(),243);
    // object moved here


    let novel = Novel::new(book, "Fiction".to_string());

    let magazine = Magazine::new(Publication{
        title: "Time".to_string(),
        author: "John Doe".to_string(),
        publication_year: 2021,
        avail_through: Resources::WeeklyDigest,
    }, "January Issue".to_string());

    let research_paper = ResearchPaper::new(Publication{
        title: "AI in Healthcare".to_string(),
        author: "Jane Doe".to_string(),
        publication_year: 2022,
        avail_through: Resources::Kindle,
    }, "AI".to_string());


    println!("Novel genre: {}", novel.genre);
    println!("Magazine issue: {}", magazine.issue);
    println!("Research paper topic: {}", research_paper.topic);
    novel.read();
}
