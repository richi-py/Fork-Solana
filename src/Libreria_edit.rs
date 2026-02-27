use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111"); // Pon aquí tu program ID real

#[program]
pub mod biblioteca {
    use super::*;

    // Crear biblioteca
    pub fn crear_biblioteca(
        context: Context<NuevaBiblioteca>,
        nombre: String,
    ) -> Result<()> {

        let owner_id = context.accounts.owner.key();

        context.accounts.biblioteca.set_inner(Biblioteca {
            owner: owner_id,
            nombre,
            libros: Vec::new(),
        });

        Ok(())
    }

    // Agregar libro
    pub fn agregar_libro(
        context: Context<NuevoLibro>,
        nombre: String,
        paginas: u16,
    ) -> Result<()> {

        require!(
            context.accounts.biblioteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let libro = Libro {
            nombre,
            paginas,
            disponible: true,
        };

        context.accounts.biblioteca.libros.push(libro);

        Ok(())
    }

    // Eliminar libro
    pub fn eliminar_libro(
        context: Context<NuevoLibro>,
        nombre: String,
    ) -> Result<()> {

        require!(
            context.accounts.biblioteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let libros = &mut context.accounts.biblioteca.libros;

        for i in 0..libros.len() {
            if libros[i].nombre == nombre {
                libros.remove(i);
                msg!("Libro {} eliminado!", nombre);
                return Ok(());
            }
        }

        Err(Errores::LibroNoExiste.into())
    }

    // Ver libros
    pub fn ver_libros(context: Context<NuevoLibro>) -> Result<()> {
        msg!("Lista de libros: {:#?}", context.accounts.biblioteca.libros);
        Ok(())
    }

    // Alternar disponibilidad
    pub fn alternar_estado(
        context: Context<NuevoLibro>,
        nombre: String,
    ) -> Result<()> {

        require!(
            context.accounts.biblioteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let libros = &mut context.accounts.biblioteca.libros;

        for libro in libros.iter_mut() {
            if libro.nombre == nombre {
                libro.disponible = !libro.disponible;

                msg!(
                    "El libro {} ahora disponible: {}",
                    nombre,
                    libro.disponible
                );

                return Ok(());
            }
        }

        Err(Errores::LibroNoExiste.into())
    }
}

#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la cuenta.")]
    NoEresElOwner,

    #[msg("El libro no existe.")]
    LibroNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct Biblioteca {
    pub owner: Pubkey,

    #[max_len(60)]
    pub nombre: String,

    #[max_len(10)]
    pub libros: Vec<Libro>,
}

#[derive(
    AnchorSerialize,
    AnchorDeserialize,
    Clone,
    InitSpace,
    PartialEq,
    Debug
)]
pub struct Libro {
    #[max_len(60)]
    pub nombre: String,
    pub paginas: u16,
    pub disponible: bool,
}

#[derive(Accounts)]
pub struct NuevaBiblioteca<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + Biblioteca::INIT_SPACE,
        seeds = [b"biblioteca", owner.key().as_ref()],
        bump
    )]
    pub biblioteca: Account<'info, Biblioteca>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevoLibro<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"biblioteca", owner.key().as_ref()],
        bump
    )]
    pub biblioteca: Account<'info, Biblioteca>,
}
