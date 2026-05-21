opaquerr::define_kind! {
    pub ErrorKind {
        /// Input is invalid.
        Invalid => "input is invalid",
        /// Anything not covered by the categories above.
        Other => "other error",
    }
}

opaquerr::define_error! {
    pub Error(ErrorKind)

    from {
        std::io::Error => ErrorKind::Other,
    }
}

fn simple() -> Result<(), Error> {
    Err(Error::simple(ErrorKind::Invalid))
}

fn with_static_message() -> Result<(), Error> {
    Err(Error::with_static_message(
        ErrorKind::Other,
        "something went wrong",
    ))
}

fn boxed() -> Result<(), Error> {
    Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "timed out").into())
}

fn main() -> Result<(), Error> {
    simple()?;
    with_static_message()?;
    boxed()?;
    Ok(())
}
