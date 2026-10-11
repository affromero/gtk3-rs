use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(string = r#"
        <interface>
          <template class="SecurityTemplateWidget" parent="GtkBox">
            <child><object class="GtkLabel" id="label">
              <property name="label">Verified label</property>
            </object></child>
            <child><object class="GtkButton" id="action_button">
              <property name="label">Verified button</property>
            </object></child>
          </template>
        </interface>
    "#)]
    pub struct Widget {
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
        #[template_child(id = "action_button")]
        pub button: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Widget {
        const NAME: &'static str = "SecurityTemplateWidget";
        type Type = super::Widget;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Widget {}
    impl WidgetImpl for Widget {}
    impl ContainerImpl for Widget {}
    impl BoxImpl for Widget {}
}

glib::wrapper! {
    pub struct Widget(ObjectSubclass<imp::Widget>)
        @extends gtk::Widget, gtk::Container, gtk::Box;
}

#[test]
fn template_binds_named_and_explicitly_renamed_children() {
    gtk::init().expect("GTK requires the test display");
    let widget: Widget = glib::Object::new();
    assert_eq!(widget.imp().label.text().as_str(), "Verified label");
    assert_eq!(
        widget.imp().button.label().as_deref(),
        Some("Verified button")
    );
}
