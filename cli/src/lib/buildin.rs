use buildin::Invoke;
use clap::Parser;

macro_rules! buildin_commands {
    ($($variant:ident => $command_ty:ty),* $(,)?) => {
        #[derive(Debug, Parser)]
        pub(crate) enum Buildin {
            $($variant($command_ty),)*
        }

        impl Buildin {
            pub(crate) fn invoke(&self) -> anyhow::Result<i32> {
                match self {
                    $(Self::$variant(cmd) => cmd.invoke(),)*
                }
            }
        }
    };
}

buildin_commands! {
    Exit => buildin_exit::Command,
    Type => buildin_type::Command,
    Pwd => buildin_pwd::Command,
    Cd => buildin_cd::Command,
    Mkdir => buildin_mkdir::Command,
    True => buildin_true::Command,
    False => buildin_false::Command,
    Touch => buildin_touch::Command,
    Cat => buildin_cat::Command,
    Head => buildin_head::Command,
    Grepr => buildin_grepr::Command,
}
