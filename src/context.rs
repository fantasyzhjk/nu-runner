use nu_cli::Print;
use nu_command::*;
use nu_protocol::{
    engine::{EngineState, StateWorkingSet},
    Config, Span, TableConfig, TableMode, UseAnsiColoring, Value,
};

pub fn create_sandboxed_context() -> EngineState {
    let mut engine_state = EngineState::new();
    let config = Config {
        show_banner: Value::bool(false, Span::unknown()),
        use_ansi_coloring: UseAnsiColoring::False,
        table: TableConfig {
            mode: TableMode::Markdown,
            ..TableConfig::default()
        },
        ..Config::default()
    };
    engine_state.set_config(config);

    let delta = {
        let mut working_set = StateWorkingSet::new(&engine_state);

        macro_rules! bind_command {
            ( $( $command:expr ),* $(,)? ) => {
                $( working_set.add_decl(Box::new($command)); )*
            };
        }

        // Help
        bind_command! {
            Help,
            HelpAliases,
            HelpExterns,
            HelpCommands,
            HelpModules,
            HelpOperators,
            HelpPipeAndRedirect,
            HelpEscapes,
        };

        // Debug
        bind_command! {
            Ast,
            Debug,
            DebugInfo,
            DebugProfile,
            Explain,
            Inspect,
            Metadata,
            MetadataAccess,
            MetadataSet,
            TimeIt,
            View,
            ViewBlocks,
            ViewFiles,
            ViewIr,
            ViewSource,
            ViewSpan,
        };

        // Charts
        bind_command! {
            Histogram
        }

        // Filters
        bind_command! {
            Shuffle,
            All,
            Any,
            Append,
            Chunks,
            Columns,
            Compact,
            Default,
            Drop,
            DropColumn,
            DropNth,
            Each,
            Enumerate,
            Every,
            Filter,
            Find,
            First,
            Flatten,
            Get,
            GroupBy,
            Headers,
            Insert,
            IsEmpty,
            IsNotEmpty,
            Interleave,
            Items,
            Join,
            Take,
            Merge,
            MergeDeep,
            Move,
            TakeWhile,
            TakeUntil,
            Last,
            Length,
            Lines,
            ParEach,
            ChunkBy,
            Prepend,
            Reduce,
            Reject,
            Rename,
            Reverse,
            Select,
            Skip,
            SkipUntil,
            SkipWhile,
            Slice,
            Sort,
            SortBy,
            SplitList,
            Tee,
            Transpose,
            Uniq,
            UniqBy,
            Upsert,
            Update,
            Values,
            Where,
            Window,
            Wrap,
            Zip,
        };

        // Path
        bind_command! {
            Path,
            PathBasename,
            PathDirname,
            PathExists,
            PathExpand,
            PathJoin,
            PathParse,
            PathRelativeTo,
            PathSplit,
            PathType,
        };

        // System
        bind_command! {
            // Complete,
            // External,
            // Exec,
            NuCheck,
            Sys,
            SysCpu,
            SysDisks,
            SysHost,
            SysMem,
            SysNet,
            SysTemp,
            SysUsers,
            UName,
            Which,
        };

        // Strings
        bind_command! {
            Char,
            Decode,
            Encode,
            DecodeHex,
            EncodeHex,
            DecodeBase32,
            EncodeBase32,
            DecodeBase32Hex,
            EncodeBase32Hex,
            DecodeBase64,
            EncodeBase64,
            DetectColumns,
            Parse,
            Split,
            SplitChars,
            SplitColumn,
            SplitRow,
            SplitWords,
            Str,
            StrCapitalize,
            StrContains,
            StrDistance,
            StrDowncase,
            StrEndswith,
            StrExpand,
            StrJoin,
            StrReplace,
            StrIndexOf,
            StrLength,
            StrReverse,
            StrStats,
            StrStartsWith,
            StrSubstring,
            StrTrim,
            StrUpcase,
            Format,
            FormatDate,
            FormatDuration,
            FormatFilesize,
        };

        // Date
        bind_command! {
            Date,
            DateFormat,
            DateHumanize,
            DateListTimezones,
            DateNow,
            DateToTimezone,
        };

        // Formats
        bind_command! {
            From,
            FromCsv,
            FromJson,
            FromMsgpack,
            FromMsgpackz,
            FromNuon,
            FromOds,
            FromSsv,
            FromToml,
            FromTsv,
            FromXlsx,
            FromXml,
            FromYaml,
            FromYml,
            To,
            ToCsv,
            ToJson,
            ToMd,
            ToMsgpack,
            ToMsgpackz,
            ToNuon,
            ToText,
            ToToml,
            ToTsv,
            Upsert,
            Where,
            ToXml,
            ToYaml,
            ToYml,
        };

        // Viewers
        bind_command! {
            Griddle,
            Table,
        };

        // Conversions
        bind_command! {
            Fill,
            Into,
            IntoBool,
            IntoBinary,
            IntoCellPath,
            IntoDatetime,
            IntoDuration,
            IntoFloat,
            IntoFilesize,
            IntoInt,
            IntoRecord,
            IntoString,
            IntoGlob,
            IntoValue,
            SplitCellPath,
        };

        // Math
        bind_command! {
            Math,
            MathAbs,
            MathAvg,
            MathCeil,
            MathFloor,
            MathMax,
            MathMedian,
            MathMin,
            MathMode,
            MathProduct,
            MathRound,
            MathSqrt,
            MathStddev,
            MathSum,
            MathVariance,
            MathLog,
        };

        // Bytes
        bind_command! {
            Bytes,
            BytesLen,
            BytesSplit,
            BytesStartsWith,
            BytesEndsWith,
            BytesReverse,
            BytesReplace,
            BytesAdd,
            BytesAt,
            BytesIndexOf,
            BytesCollect,
            BytesRemove,
            BytesBuild
        }

        // Network
        bind_command! {
            Http,
            HttpDelete,
            HttpGet,
            HttpHead,
            HttpPatch,
            HttpPost,
            HttpPut,
            HttpOptions,
            Port,
            VersionCheck,
        }
        bind_command! {
            Url,
            UrlBuildQuery,
            UrlSplitQuery,
            UrlDecode,
            UrlEncode,
            UrlJoin,
            UrlParse,
        }

        // Random
        bind_command! {
            Random,
            RandomBinary,
            RandomBool,
            RandomChars,
            RandomDice,
            RandomFloat,
            RandomInt,
            RandomUuid,
        };

        // Generators
        bind_command! {
            Cal,
            Seq,
            SeqDate,
            SeqChar,
            Generate,
        };

        // Hash
        bind_command! {
            Hash,
            HashMd5::default(),
            HashSha256::default(),
        };

        // Experimental
        //  bind_command! {
        //     IsAdmin,
        //     JobSpawn,
        //     JobList,
        //     JobKill,
        //     JobId,
        //     JobTag,
        //     Job,
        // };

        working_set.render()
    };

    let _ = engine_state.merge_delta(delta);

    engine_state
}
