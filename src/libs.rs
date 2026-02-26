use anchor_lang::prelude::*; // FrameWork para trabajar con Solana

declare_id!("");

#[program]
pub mod biblioteca{
    use super::*;  // Palabra para definir las librerias.

    pub fn crear_biblioteca() -> Result<()>{
        // Codigo...
    }
}
#[account]
#[derive(InitSpace)]
pub struct Biblioteca{
    owner: Pubkey,

    #[mas_len(60)]
    nombre: String,

    #[max_len(10)]
    libros: Vec<Libro>, // Vector que guarda una lista con libros.
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Libro{

    #[max_len(60)]
    nombre: String,
    paginas: u16,
    disponible: bool,
}

#[derive()]
pub struct NuevaBiblioteca {

    #[account(mut)]
    pub owner: Signer<'info>, // Siempre se pone 'info.

    #[account(
        init,
        pay = owner,
        space = Biblioteca:: INIT_SPACE + 8,
        seeds = [b"biblioteca", owner.key().as_ref()],
        bump // Es un tope
    )]
    pub biblioteca: Account<'info, Biblioteca>,
    pub system_program: Program<'info, System>,
}

pub struct NuevoLibro{
    pub owner: Signer<'info>,

    #[account(mut)]
    pub biblioteca: Account<'info, Biblioteca>,
}
