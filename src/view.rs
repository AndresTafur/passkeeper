extern crate gtk;

use gtk::prelude::*;
use gtk::{Button, Window, WindowType};
use service::PassKeeperService;

pub struct PassKeeperView {
    keeper: PassKeeperService,
    resource_id: gtk::Entry,
    user: gtk::Entry,
    pass: gtk::Entry,
    window: gtk::Window,
}

impl PassKeeperView {
    pub fn new(keeper: PassKeeperService) -> Self {
        if gtk::init().is_err() {
            panic!("Failed to initialize GTK.");
        }

        let window = Window::new(WindowType::Toplevel);

        window.set_default_size(640, 50);
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        Self {
            keeper,
            window,
            resource_id: gtk::Entry::new(),
            user: gtk::Entry::new(),
            pass: gtk::Entry::new(),
        }
    }

    pub fn show_gui(&self) {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
        let stack = gtk::Stack::new();

        Self::create_header(&self.window);
        self.create_query_area(&stack);
        self.create_show_area(&stack);

        vbox.add(&stack);
        self.window.add(&vbox);
        self.window.show_all();
        gtk::main();
    }

    fn create_header(window: &gtk::Window) {
        let header = gtk::HeaderBar::new();
        let button = Button::new_with_label("New");

        header.add(&button);
        header.set_show_close_button(true);
        header.set_title("Password Keeper");
        header.set_subtitle("Alpha version");
        window.set_titlebar(&header);
    }

    fn create_query_area(&self, stack: &gtk::Stack) {
        let layout = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let button = Button::new_with_label(">");
        let control_box = gtk::Box::new(gtk::Orientation::Horizontal, 2);

        self.resource_id.set_placeholder_text("id or url");

        control_box.pack_start(&self.resource_id, true, true, 2);
        control_box.pack_start(&button, false, false, 2);
        layout.pack_start(&control_box, true, true, 5);

        stack.add_named(&layout, "unlock_area");
        stack.set_transition_type(gtk::StackTransitionType::SlideUpDown);

        let bstack = stack.clone();
        let pass_keeper: PassKeeperService = self.keeper.clone();
        let url_id = self.resource_id.clone();
        let bwindow = self.window.clone();

        button.connect_clicked(move |_| {
            //seems that get_text is always some
            let id = url_id.get_text().unwrap();

            if !id.is_empty() {
                pass_keeper.get_unlocked_credentials(&id);
                bstack.set_visible_child_name("show_area");
            } else {
                bwindow.set_title("Error: master password must exist");
                let over = gtk::Overlay::new();
                over.show();
            }
        });
    }

    fn create_show_area(&self, stack: &gtk::Stack) {
        let layout = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let pass_box = gtk::Box::new(gtk::Orientation::Horizontal, 2);
        let back_btn = gtk::Button::new_with_label("<");

        let bstack = stack.clone();
        let pass_moved = self.pass.clone();

        back_btn.connect_clicked(move |_| {
            pass_moved.set_text(&"".to_string());
            bstack.set_visible_child_name("unlock_area");
        });

        self.pass.set_visibility(false);
        pass_box.pack_start(&self.user, true, true, 2);
        pass_box.pack_start(&self.pass, true, true, 2);
        pass_box.pack_start(&back_btn, false, true, 1);

        layout.pack_start(&pass_box, true, true, 5);
        stack.add_named(&layout, "show_area");
    }
}
