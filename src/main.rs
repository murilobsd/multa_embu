use select::document::Document;
use select::predicate::*;
//use std::time::Duration


#[macro_use] extern crate log;

fn main() {
    // let url = "http://sistemas.cobrasin.com.br/multas-municipe/home.action?municipio=8"
    pretty_env_logger::init();

    let doc_str = include_str!("data/embu_das_artes.html");
    let doc = Document::from(doc_str);

    pre_check_website(&doc);

    pre_form();

    submit_form();

    let doc_str = include_str!("data/embu_das_artes_lista1.html");
    let doc = Document::from(doc_str);

    pos_form(&doc);

    let doc_str = include_str!("data/embu_das_artes_view1.html");
    let doc = Document::from(doc_str);

    ger_data(&doc);
}

// ger_data
// geramos o dado no formato json mas no futuro pode ser vários outras
// estruturas
fn ger_data(doc: &Document) {
    info!("generate data");
}

// pos form
// Checamos se o formulario foi submetido corretamente e se tem informações
// pertinentes a multa caso contrário retorna a mensagem de erro.
// se tiver resultado acesse a url:
fn pos_form(doc: &Document) {
    info!("pos form");
}

// submit form
// enviamos os dados gerados pelo pre_form
fn submit_form() {
    info!("submit form");

    // let proxy = reqwest::Proxy::http("https://secure.example")?;
    // let params = [("foo", "bar"), ("baz", "quux")];
    // let client = reqwest::Client::builder()
    //     .timeout(Duration::from_millis(500))
    //     .proxy(proxy)
    //     .cookie_store(true)
    //     .build()?;
    // let res = client.post("http://httpbin.org/post")
    //     .form(&params)
    //     .send()
    //     .await?;
}

// pre form
// montamos/tratamos os dados antes de ser submetido pelo formulario
// aqui podem ser feitas validações também
fn pre_form() {
    info!("run pre_form");

    // fields names
    let field_municipio_id = "form_pesq_municipio";
    let field_renavam_id = "idrenavam";

    // fields values
    let municipio = "8";
    let renavam = "";

    let form_data = [
        (field_municipio_id, municipio),
        (field_renavam_id, renavam),
    ];

    info!("Create form data: {:?}", form_data);
}

// Pre checking
// checamos se o campo do formulario se encontra no website ainda
// caso contrario retorna algum tipo de erro que nem pensei nesse
// momento
fn pre_check_website(doc: &Document) {
    info!("run pre_check_website");

    let field_municipio_id = "form_pesq_municipio";
    let field_renavam_id = "idrenavam";

    debug!("finding field id: {}", field_municipio_id);
    assert_eq!(doc.find(Attr("id", field_municipio_id)).count(), 1);

    debug!("finding field id: {}", field_renavam_id);
    assert_eq!(doc.find(Attr("id", field_renavam_id)).count(), 1);
}
