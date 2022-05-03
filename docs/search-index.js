var searchIndex = JSON.parse('{\
"fim":{"doc":"This is a vim-like editor that provides support for …","t":[0,0,0,0,0,3,4,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,13,13,3,8,4,13,13,3,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,3,3,8,3,11,11,11,11,11,11,11,11,5,11,11,11,11,10,11,11,11,11,11,11,11,11,5,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12],"n":["config","context","editor","layout","terminal","Config","ConfigParseError","IOError","NoMatchingContext","borrow","borrow","borrow_mut","borrow_mut","fmt","fmt","from","from","from","from_file","into","into","new","to_string","try_from","try_from","try_into","try_into","type_id","type_id","context","error","line","BitSet","Bool","CommandMode","Context","ContextMessage","Float","Int","NormalMode","Str","Unit","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","forward","forward","forward","from","from","from","into","into","into","new","receive","setup","setup","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","0","0","0","0","0","Editor","borrow","borrow_mut","command_stack","draw_cmd_line","from","into","new","push_command","push_context","q_draw_cmd_line","quit","run","terminal","try_from","try_into","type_id","Colemak","Dvorak","FromFile","Layout","Qwerty","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","deshift_qwerty","from","from","from","from","from_qwerty","from_qwerty","from_qwerty","from_qwerty","from_qwerty_keycode","into","into","into","into","shift_qwerty","to_qwerty","to_qwerty","to_qwerty","to_qwerty","to_qwerty_keycode","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","Centered","Position","Size","Terminal","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","centered","centered_styles","clone","clone","clone_into","clone_into","cursor_down_by","cursor_left_by","cursor_pos","cursor_right_by","cursor_to","cursor_up_by","enter_alternate_screen","flush","from","from","from","from","height","into","into","into","into","leave_alternate_screen","move_cursor","move_cursor_to","new","new","q","q","q_move_cursor","read_key","restore_cursor","save_cursor","size","to_owned","to_owned","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","width","x","y"],"q":["fim","","","","","fim::config","","","","","","","","","","","","","","","","","","","","","","","","fim::config::ConfigParseError","","","fim::context","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","fim::context::ContextMessage","","","","","fim::editor","","","","","","","","","","","","","","","","","fim::layout","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","fim::terminal","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Module for handling configuration files that map keyboard …","A module that contains the logic for ‘Contexts’.","A module that contains the main editor logic.","A module for handling keyboard layouts.","A module for dealing with the terminal device.","Struct that represents key press to context mapping.","Enum for containing errors that might occur in parsing …","IO error (e.g. cannot open the config file)","User wants to map a key to a non-existent context.","","","","","","","","","","Create a Config from a file.","","","Create a Config from a string representing the text of the …","","","","","","","","","","","A 32-bit unsigned integer return value, intended to be …","A boolean return value.","Struct that represents fim’s CommandMode context.","Trait to represent contexts.","Enum for return values of <code>Context</code>s.","A 32-bit signed floating point return value.","A 32-bit signed integer return value.","Struct that represents fim’s NormalMode context.","A String return value.","Indicates no return value, analogous to ‘void’ in …","","","","","","","Accepts forwarded key presses.","","","","","","","","","Create a new CommandMode instance.","Receives the return value of the <code>Context</code> above it on the …","Function to setup the Context when it becomes the active …","","","","","","","","","","","","","","","","","","","Return a reference to the command history stack.","Draw the command line.","","","Create a new Editor struct.","Push a command to the command history stack.","Push a Context to the stack of contexts.","Queue the necessary <code>Command</code>s to draw the command line.","Set the quit flag.","Run the editor logic.","Return a reference to the terminal.","","","","Struct that represents the Colemak keyboard layout","Struct that represents the Dvorak keyboard layout.","Skeleton struct that represents custom, user-defined …","An interface for keyboard layouts.","Struct that represents the QWERTY keyboard layout.","","","","","","","","","Inverse of <code>shift_qwerty()</code>.","","","","","Translate a QWERTY key press into a key press from this …","","","","Translate a QWERTY <code>KeyCode</code> into a KeyCode from this …","","","","","Maps an ASCII, QWERTY press into the ASCII character that …","Translate a key press from this layout into a QWERTY key …","","","","Translate a <code>KeyCode</code> from this layout into a QWERTY …","","","","","","","","","","","","","Struct that represents styled, centered content to display.","Struct that represents a 2D position on the terminal.","Struct that represents a 2D terminal size.","Struct to represent the actual terminal the program is …","","","","","","","","","Centers text (no styles).","Creates a new <code>Centered</code> struct with the current terminal’…","","","","","Move the cursor down some amount, if able.","Move the cursor left some amount, if able.","Return a reference to this terminal’s current cursor …","Move the cursor right some amount, if able.","Set the cursor position to a location.","Move the cursor up some amount, if able.","Enter the alternate screen.","Flush the queued commands to standard output.","","","","","","","","","","Exit the alternate screen.","Move the cursor immediately.","Move the cursor to a location immediately.","Create a new Centered struct.","Create a new Terminal struct.","Queue the relevant <code>Command</code>s to print the styled content.","Queue a <code>Command</code>.","Queues the cursor move.","Poll a <code>KeyEvent</code> (blocking).","Restore the position of the cursor.","Save the position of the cursor.","Return a copy of this terminal’s size.","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,0,0,1,1,2,1,2,1,1,1,2,1,1,2,2,1,2,1,2,1,2,1,2,1,3,4,3,5,5,0,0,0,5,5,0,5,5,5,6,7,5,6,7,8,6,7,5,6,7,5,6,7,7,8,8,7,5,6,7,5,6,7,5,6,7,9,10,11,12,13,0,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,0,0,0,0,0,15,16,17,18,15,16,17,18,0,15,16,17,18,19,15,16,17,19,15,16,17,18,0,19,15,16,17,19,15,16,17,18,15,16,17,18,15,16,17,18,0,0,0,0,20,21,22,23,20,21,22,23,21,21,22,23,22,23,21,21,21,21,21,21,21,21,20,21,22,23,22,20,21,22,23,21,21,21,20,21,20,21,21,21,21,21,21,22,23,20,21,22,23,20,21,22,23,20,21,22,23,22,23,23],"f":[null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[["formatter",3]],[["error",3],["result",4,["error"]]]],[[["formatter",3]],[["error",3],["result",4,["error"]]]],[[]],[[]],[[["error",3]],["configparseerror",4]],[[["str",15]],[["configparseerror",4],["result",4,["config","configparseerror"]],["config",3]]],[[]],[[]],[[["str",15]],[["configparseerror",4],["result",4,["config","configparseerror"]],["config",3]]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[["keyevent",3],["editor",3]],[["option",4,["contextmessage"]],["result",4,["option","error"]],["error",3]]],[[["keyevent",3],["editor",3]],[["option",4,["contextmessage"]],["result",4,["option","error"]],["error",3]]],[[["keyevent",3],["editor",3]],[["option",4,["contextmessage"]],["result",4,["option","error"]],["error",3]]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["commandmode",3]],[[["contextmessage",4],["editor",3]],[["option",4,["contextmessage"]],["result",4,["option","error"]],["error",3]]],[[["editor",3]],[["result",4,["error"]],["error",3]]],[[["editor",3]],[["result",4,["error"]],["error",3]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,[[]],[[]],[[],["vec",3]],[[],[["result",4,["error"]],["error",3]]],[[]],[[]],[[],[["editor",3],["result",4,["editor","error"]],["error",3]]],[[["string",3]]],[[]],[[["bool",15]],[["result",4,["error"]],["error",3]]],[[]],[[],[["result",4,["error"]],["error",3]]],[[],["terminal",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["u8",15]],["u8",15]],[[]],[[]],[[]],[[]],[[["u8",15]],["u8",15]],[[["u8",15]],["u8",15]],[[["u8",15]],["u8",15]],[[["u8",15]],["u8",15]],[[["keycode",4]],["keycode",4]],[[]],[[]],[[]],[[]],[[["u8",15]],["u8",15]],[[["u8",15]],["u8",15]],[[["u8",15]],["u8",15]],[[["u8",15]],["u8",15]],[[["u8",15]],["u8",15]],[[["keycode",4]],["keycode",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["str",15]],["string",3]],[[["str",15],["contentstyle",3],["option",4,["contentstyle"]]],["centered",3]],[[],["size",3]],[[],["position",3]],[[]],[[]],[[["u16",15]],["terminal",3]],[[["u16",15]],["terminal",3]],[[],["position",3]],[[["u16",15]],["terminal",3]],[[["u16",15]],["terminal",3]],[[["u16",15]],["terminal",3]],[[],[["result",4,["error"]],["error",3]]],[[],[["result",4,["error"]],["error",3]]],[[]],[[]],[[]],[[]],null,[[]],[[]],[[]],[[]],[[],[["result",4,["error"]],["error",3]]],[[],[["result",4,["error"]],["error",3]]],[[["u16",15]],[["result",4,["error"]],["error",3]]],[[["stdout",3],["usize",15],["str",15],["contentstyle",3],["option",4,["contentstyle"]]],["centered",3]],[[],[["result",4,["terminal","error"]],["terminal",3],["error",3]]],[[],[["result",4,["error"]],["error",3]]],[[],[["result",4,["terminal","error"]],["error",3],["terminal",3]]],[[],[["result",4,["terminal","error"]],["error",3],["terminal",3]]],[[],[["keyevent",3],["result",4,["keyevent","error"]],["error",3]]],[[]],[[]],[[],["size",3]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null],"p":[[4,"ConfigParseError"],[3,"Config"],[13,"NoMatchingContext"],[13,"IOError"],[4,"ContextMessage"],[3,"NormalMode"],[3,"CommandMode"],[8,"Context"],[13,"Str"],[13,"BitSet"],[13,"Int"],[13,"Float"],[13,"Bool"],[3,"Editor"],[3,"Qwerty"],[3,"Dvorak"],[3,"Colemak"],[3,"FromFile"],[8,"Layout"],[3,"Centered"],[3,"Terminal"],[3,"Size"],[3,"Position"]]},\
"libfim":{"doc":"","t":[0,0,0,0,0,3,4,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,13,13,3,8,4,13,13,3,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,3,3,8,3,11,11,11,11,11,11,11,11,5,11,11,11,11,10,11,11,11,11,11,11,11,11,5,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12],"n":["config","context","editor","layout","terminal","Config","ConfigParseError","IOError","NoMatchingContext","borrow","borrow","borrow_mut","borrow_mut","fmt","fmt","from","from","from","from_file","into","into","new","to_string","try_from","try_from","try_into","try_into","type_id","type_id","context","error","line","BitSet","Bool","CommandMode","Context","ContextMessage","Float","Int","NormalMode","Str","Unit","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","forward","forward","forward","from","from","from","into","into","into","new","receive","setup","setup","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","0","0","0","0","0","Editor","borrow","borrow_mut","command_stack","draw_cmd_line","from","into","new","push_command","push_context","q_draw_cmd_line","quit","run","terminal","try_from","try_into","type_id","Colemak","Dvorak","FromFile","Layout","Qwerty","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","deshift_qwerty","from","from","from","from","from_qwerty","from_qwerty","from_qwerty","from_qwerty","from_qwerty_keycode","into","into","into","into","shift_qwerty","to_qwerty","to_qwerty","to_qwerty","to_qwerty","to_qwerty_keycode","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","Centered","Position","Size","Terminal","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","centered","centered_styles","clone","clone","clone_into","clone_into","cursor_down_by","cursor_left_by","cursor_pos","cursor_right_by","cursor_to","cursor_up_by","enter_alternate_screen","flush","from","from","from","from","height","into","into","into","into","leave_alternate_screen","move_cursor","move_cursor_to","new","new","q","q","q_move_cursor","read_key","restore_cursor","save_cursor","size","to_owned","to_owned","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","width","x","y"],"q":["libfim","","","","","libfim::config","","","","","","","","","","","","","","","","","","","","","","","","libfim::config::ConfigParseError","","","libfim::context","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","libfim::context::ContextMessage","","","","","libfim::editor","","","","","","","","","","","","","","","","","libfim::layout","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","libfim::terminal","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Module for handling configuration files that map keyboard …","A module that contains the logic for ‘Contexts’.","A module that contains the main editor logic.","A module for handling keyboard layouts.","A module for dealing with the terminal device.","Struct that represents key press to context mapping.","Enum for containing errors that might occur in parsing …","IO error (e.g. cannot open the config file)","User wants to map a key to a non-existent context.","","","","","","","","","","Create a Config from a file.","","","Create a Config from a string representing the text of the …","","","","","","","","","","","A 32-bit unsigned integer return value, intended to be …","A boolean return value.","Struct that represents fim’s CommandMode context.","Trait to represent contexts.","Enum for return values of <code>Context</code>s.","A 32-bit signed floating point return value.","A 32-bit signed integer return value.","Struct that represents fim’s NormalMode context.","A String return value.","Indicates no return value, analogous to ‘void’ in …","","","","","","","Accepts forwarded key presses.","","","","","","","","","Create a new CommandMode instance.","Receives the return value of the <code>Context</code> above it on the …","Function to setup the Context when it becomes the active …","","","","","","","","","","","","","","","","","","","Return a reference to the command history stack.","Draw the command line.","","","Create a new Editor struct.","Push a command to the command history stack.","Push a Context to the stack of contexts.","Queue the necessary <code>Command</code>s to draw the command line.","Set the quit flag.","Run the editor logic.","Return a reference to the terminal.","","","","Struct that represents the Colemak keyboard layout","Struct that represents the Dvorak keyboard layout.","Skeleton struct that represents custom, user-defined …","An interface for keyboard layouts.","Struct that represents the QWERTY keyboard layout.","","","","","","","","","Inverse of <code>shift_qwerty()</code>.","","","","","Translate a QWERTY key press into a key press from this …","","","","Translate a QWERTY <code>KeyCode</code> into a KeyCode from this …","","","","","Maps an ASCII, QWERTY press into the ASCII character that …","Translate a key press from this layout into a QWERTY key …","","","","Translate a <code>KeyCode</code> from this layout into a QWERTY …","","","","","","","","","","","","","Struct that represents styled, centered content to display.","Struct that represents a 2D position on the terminal.","Struct that represents a 2D terminal size.","Struct to represent the actual terminal the program is …","","","","","","","","","Centers text (no styles).","Creates a new <code>Centered</code> struct with the current terminal’…","","","","","Move the cursor down some amount, if able.","Move the cursor left some amount, if able.","Return a reference to this terminal’s current cursor …","Move the cursor right some amount, if able.","Set the cursor position to a location.","Move the cursor up some amount, if able.","Enter the alternate screen.","Flush the queued commands to standard output.","","","","","","","","","","Exit the alternate screen.","Move the cursor immediately.","Move the cursor to a location immediately.","Create a new Centered struct.","Create a new Terminal struct.","Queue the relevant <code>Command</code>s to print the styled content.","Queue a <code>Command</code>.","Queues the cursor move.","Poll a <code>KeyEvent</code> (blocking).","Restore the position of the cursor.","Save the position of the cursor.","Return a copy of this terminal’s size.","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,0,0,1,1,2,1,2,1,1,1,2,1,1,2,2,1,2,1,2,1,2,1,2,1,3,4,3,5,5,0,0,0,5,5,0,5,5,5,6,7,5,6,7,8,6,7,5,6,7,5,6,7,7,8,8,7,5,6,7,5,6,7,5,6,7,9,10,11,12,13,0,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,14,0,0,0,0,0,15,16,17,18,15,16,17,18,0,15,16,17,18,19,15,16,17,19,15,16,17,18,0,19,15,16,17,19,15,16,17,18,15,16,17,18,15,16,17,18,0,0,0,0,20,21,22,23,20,21,22,23,21,21,22,23,22,23,21,21,21,21,21,21,21,21,20,21,22,23,22,20,21,22,23,21,21,21,20,21,20,21,21,21,21,21,21,22,23,20,21,22,23,20,21,22,23,20,21,22,23,22,23,23],"f":[null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[["error",3]]],[[["str",15]],[["result",4,["config","configparseerror"]],["config",3],["configparseerror",4]]],[[]],[[]],[[["str",15]],[["result",4,["config","configparseerror"]],["config",3],["configparseerror",4]]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[["editor",3],["keyevent",3]],[["option",4,["contextmessage"]],["result",6,["option"]]]],[[["editor",3],["keyevent",3]],[["option",4,["contextmessage"]],["result",6,["option"]]]],[[["editor",3],["keyevent",3]],[["option",4,["contextmessage"]],["result",6,["option"]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["commandmode",3]],[[["contextmessage",4],["editor",3]],[["option",4,["contextmessage"]],["result",6,["option"]]]],[[["editor",3]],["result",6]],[[["editor",3]],["result",6]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,[[]],[[]],[[],["vec",3]],[[],["result",6]],[[]],[[]],[[],[["result",6,["editor"]],["editor",3]]],[[["string",3]]],[[["context",8]]],[[["bool",15]],["result",6]],[[]],[[],["result",6]],[[],["terminal",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["u8",15]],["u8",15]],[[]],[[]],[[]],[[]],[[["u8",15]],["u8",15]],[[["u8",15]],["u8",15]],[[["u8",15]],["u8",15]],[[["u8",15]],["u8",15]],[[["keycode",4]],["keycode",4]],[[]],[[]],[[]],[[]],[[["u8",15]],["u8",15]],[[["u8",15]],["u8",15]],[[["u8",15]],["u8",15]],[[["u8",15]],["u8",15]],[[["u8",15]],["u8",15]],[[["keycode",4]],["keycode",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["str",15]],["string",3]],[[["option",4,["contentstyle"]],["str",15],["contentstyle",3]],["centered",3]],[[],["size",3]],[[],["position",3]],[[]],[[]],[[["u16",15]]],[[["u16",15]]],[[],["position",3]],[[["u16",15]]],[[["u16",15]]],[[["u16",15]]],[[],["result",6]],[[],["result",6]],[[]],[[]],[[]],[[]],null,[[]],[[]],[[]],[[]],[[],["result",6]],[[],["result",6]],[[["u16",15]],["result",6]],[[["stdout",3],["usize",15],["str",15],["option",4,["contentstyle"]],["contentstyle",3]]],[[],["result",6]],[[],["result",6]],[[],["result",6]],[[],["result",6]],[[],[["keyevent",3],["result",6,["keyevent"]]]],[[]],[[]],[[],["size",3]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null],"p":[[4,"ConfigParseError"],[3,"Config"],[13,"NoMatchingContext"],[13,"IOError"],[4,"ContextMessage"],[3,"NormalMode"],[3,"CommandMode"],[8,"Context"],[13,"Str"],[13,"BitSet"],[13,"Int"],[13,"Float"],[13,"Bool"],[3,"Editor"],[3,"Qwerty"],[3,"Dvorak"],[3,"Colemak"],[3,"FromFile"],[8,"Layout"],[3,"Centered"],[3,"Terminal"],[3,"Size"],[3,"Position"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};