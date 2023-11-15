package org.unitmesh.inference

import android.annotation.SuppressLint
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import org.unitmesh.bindgen.Document
import org.unitmesh.bindgen.InMemoryEmbeddingStore
import org.unitmesh.bindgen.Metadata
import org.unitmesh.bindgen.initSemantic
import org.unitmesh.inference.ui.theme.InferenceExampleTheme
import java.io.InputStream

class MainActivity : ComponentActivity() {
    @SuppressLint("UnsafeDynamicallyLoadedCode")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val store = InMemoryEmbeddingStore()

        val tokenizerStream: InputStream = assets.open("model/tokenizer.json")
        val onnxStream: InputStream = assets.open("model/model.onnx")

        val model: List<UByte> = onnxStream.readBytes().map { it.toUByte() }
        val tokenizer: List<UByte> = tokenizerStream.readBytes().map { it.toUByte() }

        val semantic = initSemantic(model, tokenizer)
        val list = semantic.embed("hello")
        println(list)

        val metadata = Metadata(mapOf())
        store.add("hello", listOf(0.1f), Document("", metadata, "", listOf()))
        val findRelevant = store.findRelevant(listOf(0.1f), 1, 0.5f)
        println(findRelevant)

        setContent {
            InferenceExampleTheme {
                // A surface container using the 'background' color from the theme
                Surface(modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background) {
                    Greeting("Android")
                }
            }
        }
    }
}

@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {
    Text(
        text = "Hello $name!",
        modifier = modifier
    )
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    InferenceExampleTheme {
        Greeting("Android")
    }
}