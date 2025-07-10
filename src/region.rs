use clap::ValueEnum;

#[derive(Clone, Debug, ValueEnum, Default)]
pub enum Region {
    #[default]
    Europe,
    NorthAmerica,
    Australia,
}
