var searchIndex = JSON.parse('{\
"pangalacticcc":{"doc":"A helpful calculator for hitchiking merchants all across …","t":[3,5,5,11,11,11,11,5,11,5,0,5,0,11,11,11,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,3,11,11,11,11,11,11,11,11,11,11,5,5,5,5,11,11,11,11,11,11,11,11,5,5,5,5,5,11,11,11,11,11,11,11,11,11,11],"n":["Config","answer_how_many_credits","answer_how_much","borrow","borrow_mut","fmt","from","get_args","into","open","roman","run","textprocessing","try_from","try_into","type_id","ParseRomanNumeralError","ROMAN_VALUES","Roman","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone_into","deref","eq","fmt","fmt","fmt","fmt","from","from","from","from","from_str","get_representation","get_value","into","into","into","to_owned","to_string","to_string","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","MapAlienNumeralError","ParseSentenceError","borrow","borrow","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","eq","eq","extract_amount_credits_from_sentence","extract_amounts_from_sentence","extract_unit_values_from_sentence","extract_units_from_sentence","fmt","fmt","fmt","fmt","from","from","into","into","is_numeral_info","is_question_how_many_credits","is_question_how_much","is_unit_info","numerals_to_roman","to_owned","to_owned","to_string","to_string","try_from","try_from","try_into","try_into","type_id","type_id"],"q":["pangalacticcc","","","","","","","","","","","","","","","","pangalacticcc::roman","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","pangalacticcc::textprocessing","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Holds the path of the file to be processed as String.","Returns response to input asking “how many credits is ……","Returns response to input asking “how much is …” as …","","","","Returns the argument unchanged.","Parses command line arguments","Calls <code>U::from(self)</code>.","Returns a BufReader for <code>path</code> on success. If <code>path</code> is <code>&quot;-&quot;</code>,  …","","Runs the program on provided config. Output is printed to …","","","","","Occurs when the input could not be converted to Roman, …","","Represents a Roman number","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Converts from unsigned integer to Roman if num is within …","Returns the argument unchanged.","Converts from &amp;str to Roman. The symbols “I”, “X”, …","Returns representation in Roman numerals as String","Returns value as integer","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","Occurs when alien numeral could not be mapped","Occurs when the input could not be parsed, e.g. due to …","","","","","","","","","","","Returns a amount of Credits extracted from a sentence or …","Returns a Result with the amount extracted from a sentence …","Returns Result for credit conversion rate for unit …","Returns a Result with the unit extracted from a sentence …","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Returns true if sentence is statement about numerals","Returns true if questions asks for how many credits, and …","Returns true if questions asks how much is","Returns true if sentence is statement about units, and a …","Returns (alien_numeral, roman_numeral) tuple from a …","","","","","","","","","",""],"i":[0,0,0,1,1,1,1,0,1,0,0,0,0,1,1,1,0,0,0,2,3,4,2,3,4,3,3,2,3,3,3,4,4,2,3,4,4,4,4,4,2,3,4,3,3,4,2,3,4,2,3,4,2,3,4,0,0,5,6,5,6,5,6,5,6,5,6,0,0,0,0,5,5,6,6,5,6,5,6,0,0,0,0,0,5,6,5,6,5,6,5,6,5,6],"f":[null,[[["hashmap",3],["hashmap",3],["str",0]],["string",3]],[[["hashmap",3],["str",0]],["string",3]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0],["formatter",3]],["result",6]],[[]],[[],["result",4,[["config",3],["box",3,[["error",8]]]]]],[[]],[[["str",0]],["result",4,[["box",3,[["bufread",8]]],["box",3,[["error",8]]]]]],null,[[["config",3]],["result",4,[["box",3,[["error",8]]]]]],null,[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["parseromannumeralerror",3]],[[["",0],["",0]]],[[["",0]],["hashmap",3]],[[["",0],["parseromannumeralerror",3]],["bool",0]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[["u32",0]]],[[]],[[["str",0]],["result",4]],[[["",0]],["string",3]],[[["",0]],["i32",0]],[[]],[[]],[[]],[[["",0]]],[[["",0]],["string",3]],[[["",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["parsesentenceerror",3]],[[["",0]],["mapaliennumeralerror",3]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["parsesentenceerror",3]],["bool",0]],[[["",0],["mapaliennumeralerror",3]],["bool",0]],[[["str",0]],["option",4,[["i32",0]]]],[[["hashmap",3],["str",0]],["result",4,[["i32",0],["box",3,[["error",8]]]]]],[[["hashmap",3],["str",0]],["result",4,[["box",3,[["error",8]]]]]],[[["str",0]],["result",4,[["string",3],["box",3,[["error",8]]]]]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[["str",0]],["bool",0]],[[["str",0]],["bool",0]],[[["str",0]],["bool",0]],[[["str",0]],["bool",0]],[[["str",0]],["option",4]],[[["",0]]],[[["",0]]],[[["",0]],["string",3]],[[["",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]]],"p":[[3,"Config"],[3,"ROMAN_VALUES"],[3,"ParseRomanNumeralError"],[3,"Roman"],[3,"ParseSentenceError"],[3,"MapAlienNumeralError"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};