use neon::prelude::*;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());

#[derive(Clone)]
pub struct QueryEngine {
    datamodel: String,
}

impl Finalize for QueryEngine {}

impl QueryEngine {
    pub fn new(datamodel: String) -> Self {
        Self { datamodel }
    }

    pub async fn query(&self) -> String {
        let data = serde_json::json!({
            "findFirstBooking": {
                "id": "ckovh15xa104945sj64rdk8oas",
                "name": "1883da9ff9152",
                "forename": "221c99bedc6a4",
                "description": "8bf86b62ce6a",
                "email": "9d57a869661cc",
                "phone": "7e0c58d147215",
                "arrivalDate": -92229669,
                "departureDate": 202138795,
                "price": -1592700387,
                "advance": -369294193,
                "advanceDueDate": 925000428,
                "kids": 520124290,
                "adults": 1160258464,
                "status": "NO_PAYMENT",
                "nourishment": "BB",
                "createdAt": "2021-05-19T12:58:37.246Z",
                "room": { "id": "ckovh15xa104955sj6r2tqaw1c", "name": "38683b87f2664" }
            }
        });

        serde_json::to_string(&data).unwrap()
    }
}

impl QueryEngine {
    fn js_new(mut cx: FunctionContext) -> JsResult<JsBox<QueryEngine>> {
        let datamodel = cx.argument::<JsString>(0)?.value(&mut cx);
        let engine = Self::new(datamodel);

        Ok(cx.boxed(engine))
    }

    fn js_query(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let callback = cx.argument::<JsFunction>(0)?.root(&mut cx);
        let queue = cx.queue();

        let engine = cx.this().downcast_or_throw::<JsBox<QueryEngine>, _>(&mut cx)?;
        let engine: QueryEngine = (**engine).clone();

        RUNTIME.spawn(async move {
            let res = engine.query().await;

            queue.send(move |mut cx| {
                let callback = callback.into_inner(&mut cx);
                let this = cx.undefined();

                let args = vec![cx.null().upcast::<JsValue>(), cx.string(res).upcast()];

                callback.call(&mut cx, this, args)?;

                Ok(())
            })
        });

        Ok(cx.undefined())
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("engineNew", QueryEngine::js_new)?;
    cx.export_function("engineQuery", QueryEngine::js_query)?;

    Ok(())
}
