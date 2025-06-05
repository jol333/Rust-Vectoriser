use iced::widget::{button, column, container, text};
use iced::{executor, Application, Command, Element, Length, Settings, Theme};
use rfd::FileDialog;
use std::path::PathBuf;

// Define the application state
struct ImageVectoriser {
    selected_image: Option<PathBuf>,
    status_message: String,
}

impl Default for ImageVectoriser {
    fn default() -> Self {
        Self {
            selected_image: None,
            status_message: String::from("No image selected"),
        }
    }
}

// Define the messages that can be sent in the application
#[derive(Debug, Clone)]
enum Message {
    OpenFilePicker,
    FileSelected(Option<PathBuf>),
}

impl Application for ImageVectoriser {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Rust Vectoriser")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::OpenFilePicker => {
                return Command::perform(
                    async {
                        let file_dialog = FileDialog::new()
                            .add_filter("Images", &["jpg", "jpeg", "png"])
                            .set_title("Select an image")
                            .pick_file();
                        
                        file_dialog
                    },
                    Message::FileSelected,
                );
            }
            Message::FileSelected(file_path) => {
                match file_path {
                    Some(path) => {
                        self.status_message = format!("Selected: {}", path.display());
                        self.selected_image = Some(path);
                    },
                    None => {
                        self.status_message = String::from("No file selected");
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        // Create a button to open file picker
        let pick_button = button("Select Image")
            .on_press(Message::OpenFilePicker);

        // Display status message
        let status = text(&self.status_message);

        // Create a column layout with button and status
        let content = column![pick_button, status]
            .spacing(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(iced::Alignment::Center);

        // Wrap it in a container for padding
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn main() -> iced::Result {
    ImageVectoriser::run(Settings::default())
}