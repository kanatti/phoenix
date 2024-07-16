#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let ctx = Rc::new(QueryContext::new());

    // let planner = QueryPlanner::new(ctx.clone());
    // let optimizer = QueryOptimizer::new(ctx.clone());
    // let executor = QueryExecutor::new(ctx.clone());

    // let engine = QueryEngine::new(planner, optimizer, executor);
    // let handler = SimpleQueryHandler::new(engine);

    // handler.handle_query("SELECT * FROM table_name WHERE condition").await?;
    Ok(())
}