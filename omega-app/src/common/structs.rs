#[derive(Debug)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub data: Option<Vec<u8>>,
}

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
    descricao: String,
    preco: f32,
    quantidade: i32,
}

