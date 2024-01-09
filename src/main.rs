use std::io::Read;

use openai_rs::*;

#[tokio::main]
async fn main() {
    let mut client = OpenAIAccount::new(GptModel::Gpt4, 0.2, false, None, None).await.expect("Initial startup");
    client.cache.clear();

    let mut content = String::new();
    let f = std::fs::File::open("inputs.txt").expect("to be able to read the file").read_to_string(&mut content).unwrap();
    let cite_blocks: Vec<&str> = content.split("\n").collect();

    for block in cite_blocks {
        let res = client.get_completion(format!("{block}  
        type Props = {{
            authors: string[]; // L. M. First
            year: number;
            title: string;
            journal: string;
            edition?: number;
            articleNumber: number;
            pages?: {{
                start: number;
                end: number;
            }};
        }});  

        Extract the above information and return the following JSX tag:
        {{ title:   <Citation {{...props}} /> }}").as_str())
            .await
            .expect("success of chat completion");

        if res.from_cache { println!("{} from cache", res.response.id); }
    }

    let meta_res = client.meta_complete_cache("Provide a variable declaration of these objects as a single object: const references = { ... };")
        .await
        .expect("success of meta completion");

    println!("{}", meta_res.response.choices[0].to_owned().message.content.unwrap())
}
