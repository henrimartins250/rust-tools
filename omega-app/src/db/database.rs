use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

pub fn init() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (), // empty list of parameters.
    )?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person?);
    }
    Ok(())
}

use chrono::{NaiveDateTime};

pub fn pet_shop_db() -> Result<()>  {


/*
    Atividade 01 
Considere um sistema de Pet Shop com as tabelas Cliente, Animal, Servico_Animal, Funcionario e Produto, onde: 

Cliente contém CPF, nome, telefone e endereço. 

Animal contém código do animal, nome, tipo e CPF do cliente dono. 

Servico_Animal registra os serviços prestados, com código do serviço, CPF do cliente, CPF do funcionário, código do animal, descrição do serviço, preço e data. 

Funcionario contém CPF, nome, endereço, telefone e função. 

Produto contém código do produto, nome, descrição, preço e quantidade em estoque.

Crie uma VIEW chamada Boletim_Servicos que exiba os seguintes dados para cada serviço prestado: 
Nome do cliente 
Nome do animal 
Descrição do serviço 
Nome do funcionário responsável 
Preço do serviço 
Data do serviço 
A VIEW deve relacionar as tabelas para mostrar o histórico completo dos serviços prestados a cada animal. 
*/
    pub struct Cliente {
        cpf: String,
        nome: String,
        telefone: String,
        endereco: String,
    }

    pub struct Animal {
        codigo_animal: i32,
        nome: String,
        tipo: String,
        cpf_dono: String,
    }

    pub struct ServicoAnimal {
        codigo_servico: i32,
        cpf_cliente: String,
        cpf_funcionario: String,
        codigo_animal: i32,
        descricao: String,
        preco: f32,
        data: NaiveDateTime,
    }

    pub struct Funcionario {
        cpf: String,
        nome: String,
        endereco: String,
        telefone: String,
        funcao: String,
    }

    pub struct Produto {
        codigo_produto: i32,
        nome: String,
        descrição: String,
        preco: f32,
        quantidade: i32,
    }

    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (), // empty list of parameters.
    )?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person?);
    }
    Ok(())
}

pub fn rh_db() {

}

pub fn academic_db() {

}