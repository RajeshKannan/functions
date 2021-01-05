pub trait ExecutionContext {
    fn name(&self) -> &str {
        "DefaultExecutor"
    }
    fn log(&self, message: &str) {
        println!("{}-{}", self.name(), message)
    }
}
pub trait BusinessLogic {
    fn execute(&self, input: &str, execution_context: &impl ExecutionContext) -> String;
}


#[cfg(test)]
mod tests {
    use crate::{BusinessLogic, ExecutionContext};

    struct GreeterFunction;
    impl BusinessLogic for GreeterFunction {
        fn execute(&self, input: &str, execution_context: &impl ExecutionContext) -> String {
            execution_context.log(format!("received a request {}", input).as_str());
            format!("Hello, {}", input)
        }
    }

    struct StandaloneContext;
    impl ExecutionContext for StandaloneContext {
        fn name(&self) -> &str {
            "StandaloneExecutor"
        }
    }
    #[test]
    fn it_works_for_default_execution_context() {
        let greeter = GreeterFunction{};
        let context = StandaloneContext {};
        assert_eq!("Hello, Rajesh", greeter.execute("Rajesh", &context));
    }
}
