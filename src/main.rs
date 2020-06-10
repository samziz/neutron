use cpp_core::{Ptr, Ref, StaticUpcast};
use qt_core::{qs, slot, ContextMenuPolicy, QBox, QObject, QPoint, SlotNoArgs};
use qt_widgets::{
    QAction, QApplication, QFormLayout, QLineEdit, QMenu, QMessageBox, QPushButton, QTableWidget,
    QTableWidgetItem, QVBoxLayout, QWidget, SlotOfQPoint, SlotOfQTableWidgetItemQTableWidgetItem,
};
use qt_widgets::qt_core::AlignmentFlag;
use std::rc::Rc;

struct Form {
    widget: QBox<QWidget>,
    line_edit: QBox<QLineEdit>,
}

impl StaticUpcast<QObject> for Form {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.widget.as_ptr().static_upcast()
    }
}

impl Form {
    fn new() -> Rc<Form> {
        unsafe {
            let widget = QWidget::new_0a();
            let layout = QFormLayout::new_1a(&widget);

            let line_edit = QLineEdit::new();
            layout.add_widget(&line_edit);

            widget.show();

            widget.set_minimum_height(800);
            widget.set_minimum_width(1200);

            let this = Rc::new(Self {
                widget,
                line_edit,
            });
            this.init();
            this
        }
    }

    unsafe fn init(self: &Rc<Self>) {
        self.line_edit
            .text_edited();
    }
}

fn main() {
    QApplication::init(|_| unsafe {
        let _form = Form::new();
        QApplication::exec()
    })
}
