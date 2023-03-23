use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use mysql::*;
use mysql::prelude::*;
use serde::{Serialize,Deserialize};
#[derive(Serialize, Deserialize,Debug)]
struct Product {
    id: String,
    studname: String,
    age : String,
    sex:String,
    class: String,
    stdid : String
}
#[get("/")]
async fn select() -> impl Responder{    
    let url ="mysql://root:aA111111@127.0.0.1:3306/test";
    let pool=Pool::new(url).unwrap();
    let mut conn=pool.get_conn().unwrap();
   
    let res = conn.query_map("select * from student",|
    (Product_id ,Product_studname ,Product_age  ,prelude_sex,Product_class ,Product_stdid)
    |Product{
        id:Product_id,
        studname:Product_studname,
        age:Product_age,
        sex:prelude_sex,
        class:Product_class,
        stdid:Product_stdid
    })
    .expect("Query failed.");
    
    println!("{:?}", res);
    let ans= serde_json::to_value(&res).unwrap();
    println!("123456={:?}",ans);
    HttpResponse::Ok().json(ans)
}

#[get("/delete/{userid}")]
async fn delete(path: web::Path<String>) -> impl Responder {
    let url ="mysql://root:aA111111@127.0.0.1:3306/test";
    let pool=Pool::new(url).unwrap();
    let mut conn=pool.get_conn().unwrap();
    let path = path.into_inner();
    let id = path;
    let sql = "delete from student where id = :id";
    conn.exec_drop(&sql, params!{
        "id" => id,
    }).unwrap();
    format!("123")
}
#[get("/insert/{id}/{student_name}/{age}/{sex}/{class}/{student_ID}")]
async fn insert(path: web::Path<(String,String,String,String,String,String,)>) -> impl Responder {
    let url ="mysql://root:aA111111@127.0.0.1:3306/test";
    let pool=Pool::new(url).unwrap();
    let mut conn=pool.get_conn().unwrap();
    let path = path.into_inner();
    let id = path.0;
    let student_name =path.1;
    let age =path.2;
    let sex = path.3;
    let class =path.4;
    let student_id =path.5;

    let sql = "INSERT INTO student (id,student_name,age,sex,class,student_id) VALUES (:id,:student_name,:age,:sex,:class,:student_id)";
    conn.exec_drop(&sql, params!{
        "id" => id,
        "student_name" => student_name,
        "age" => age,
        "sex" => sex,
        "class" => class,
        "student_id" => student_id,
    }).unwrap();
    format!("123")
}
#[get("/update/{sex1}/{sex2}")]
async fn update(path: web::Path<(String,String)>) -> impl Responder {
    let url ="mysql://root:aA111111@127.0.0.1:3306/test";
    let pool=Pool::new(url).unwrap();
    let mut conn=pool.get_conn().unwrap();
    let path = path.into_inner();
    let sex1 = path.0;
    let sex2 =path.1;
    let sql = "update student set sex = :sex1 where sex = :sex2";
    conn.exec_drop(&sql, params!{
        "sex1" => sex1,
        "sex2" => sex2,
    }).unwrap();
    format!("123")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(select)
            .service(delete)
            .service(insert)
            .service(update)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}