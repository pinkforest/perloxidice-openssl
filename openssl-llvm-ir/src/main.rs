use llvm_ir::Module;
use llvm_ir::function::Function;
use llvm_ir::function::Parameter;
use llvm_ir::Name;
use llvm_ir::types::Type;

fn format_type(t: &Type) -> String {
    match *t {
        Type::PointerType{addr_space: _ } => "ptr".to_string(),
        Type::IntegerType{bits: b} => format!("I{b}"),
        Type::VoidType => "void".to_string(),
        _ => "todo!()".to_string(),
    }   
}

fn format_name(n: &Name) -> String {
    match n {
        Name::Name(box_name) => *box_name.clone(),
        Name::Number(siz) => format!("[{siz}]"),
    }
}

fn format_parameter(p: &Parameter) -> String {
    // TODO Attributes = .attributes = https://docs.rs/llvm-ir/latest/llvm_ir/function/enum.ParameterAttribute.html
    let param_name = format_name(&p.name);
    let param_type = format_type(&p.ty);
    format!("{}: {}", param_name, param_type).to_string()
}

fn print_function(f: &Function) {

    let debug_loc_status: String = match &f.debugloc {
        Some(d) => format!("[{}]", d.line),
        None => "-".to_string(),
    };
    
    let formatted_input_parameters: Vec<String> = f.parameters.iter().map(format_parameter).collect();

    let function_analysis = llvm_ir_analysis::FunctionAnalysis::new(f);
    let control_flow_graph = function_analysis.control_flow_graph();
    
    println!(" ?<{debug_loc_status}> [{}]{} ({}) -> {}", control_flow_graph.entry(),f.name, formatted_input_parameters.join(","), format_type(&f.return_type));

    let preds: Vec<String> = control_flow_graph.preds(control_flow_graph.entry()).map(format_name).collect();
    let succs: Vec<String> = control_flow_graph.succs(control_flow_graph.entry()).map(|n| format!("{:?}", n) ).collect();
    let preds_of_return: Vec<String> = control_flow_graph.preds_of_return().map(|n| format!("{:?}", n) ).collect();


    println!("  <<< preds = {:?}", preds);
    println!("  --> succs = {:?}", succs);
    println!("  <== preds_of_return = {:?}", preds_of_return);
    
    
//    println!(" ::: {:?}", f.basic_blocks);  
    //    println!("  >>> u.parameters : {:?}", f.parameters);
    // println!("  >>> f.parameters : {:?}", formatted_parameters);
//     println!("  <<< return_Type: {:?}", f.return_type);
//    println!("    >> func_attrs : {:?}", f.function_attributes);
//    println!("   << return_attrs: {:?}", f.return_attributes);
}

fn main() {
    
    let file = std::env::args().last().unwrap();

    let module = Module::from_bc_path(file).unwrap();

    let target_triple = match &module.target_triple {
        Some(t) => t.clone(),
        None => "None".to_string(),
    };

    // Debug impl would be nice :)
    println!("===== llvm_ir::Module");
    println!("name             = {}", &module.name);
    println!("source_file_name = {}", &module.source_file_name);
    println!("target_triple    = {}", target_triple);


    println!("  --> Functions");
    module.functions.iter().for_each(print_function);

    //println!("functions       ?= {}", &module.functions);
    //println!("data_layout     ?= {:?}", &module.data_layout);
    

}
