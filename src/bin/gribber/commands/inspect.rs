use clap::{App, Arg, ArgMatches, SubCommand};
use console::{Style, Term};

use grib::context::{SectionInfo, TemplateInfo};

use crate::cli;

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("inspect")
        .about("Inspects and describes the data structure")
        .arg(
            Arg::with_name("sections")
                .help("Prints sections constructing the GRIB message")
                .short("s")
                .long("sections"),
        )
        .arg(
            Arg::with_name("templates")
                .help("Prints templates used in the GRIB message")
                .short("t")
                .long("templates"),
        )
        .arg(Arg::with_name("file").required(true))
        .after_help(
            "\
This subcommand is mainly targeted at (possible) developers and
engineers, who wants to understand the data structure for the purpose
of debugging, enhancement, and education.\
",
        )
}

pub fn exec(args: &ArgMatches<'static>) -> Result<(), cli::CliError> {
    let file_name = args.value_of("file").unwrap();
    let grib = cli::grib(file_name)?;

    let mut view = InspectView::new();
    if args.is_present("sections") {
        view.add(InspectItem::Sections(grib.sections()));
    }
    if args.is_present("templates") {
        let tmpls = grib.list_templates();
        view.add(InspectItem::Templates(tmpls));
    }
    if view.items.len() == 0 {
        view.add(InspectItem::Sections(grib.sections()));
        let tmpls = grib.list_templates();
        view.add(InspectItem::Templates(tmpls));
    }

    let user_attended = console::user_attended();

    let term = Term::stdout();
    let (height, _width) = term.size();
    if view.num_lines() > height.into() {
        cli::start_pager();
    }

    if user_attended {
        console::set_colors_enabled(true);
    }

    let with_header = view.with_headers();
    let mut items = view.items.into_iter().peekable();
    loop {
        let item = match items.next() {
            None => break,
            Some(i) => i,
        };

        if with_header {
            let yellow = Style::new().yellow().bold();
            let s = format!("{}:", item.title());
            println!("{}", yellow.apply_to(s));
        }

        match item {
            InspectItem::Sections(sects) => {
                for sect in sects.iter() {
                    println!("{}", sect);
                }
            }
            InspectItem::Templates(tmpls) => {
                for tmpl in tmpls.iter() {
                    println!("{}", tmpl);
                }
            }
        }

        if let Some(_) = items.peek() {
            println!("");
        }
    }

    Ok(())
}

struct InspectView<'i> {
    items: Vec<InspectItem<'i>>,
}

impl<'i> InspectView<'i> {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn add(&mut self, item: InspectItem<'i>) {
        self.items.push(item);
    }

    fn with_headers(&self) -> bool {
        !(self.items.len() < 2)
    }

    fn num_lines(&self) -> usize {
        let mut count = 0;
        for item in self.items.iter() {
            if self.with_headers() {
                count += 1;
            }
            count += item.len();
        }
        count += self.items.len() - 1; // empty lines
        count
    }
}

enum InspectItem<'i> {
    Sections(&'i Box<[SectionInfo]>),
    Templates(Vec<TemplateInfo>),
}

impl<'i> InspectItem<'i> {
    fn title(&self) -> &'static str {
        match self {
            InspectItem::Sections(_) => "Sections",
            InspectItem::Templates(_) => "Templates",
        }
    }

    fn len(&self) -> usize {
        match self {
            InspectItem::Sections(sects) => sects.len(),
            InspectItem::Templates(tmpls) => tmpls.len(),
        }
    }
}
