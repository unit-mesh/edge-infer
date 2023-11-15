package org.unitmesh.inference

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
import org.unitmesh.inference.ui.theme.InferenceExampleTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val store = InMemoryEmbeddingStore()
        val metadata = Metadata(mapOf())
        store.add("hello", listOf(0.1f), Document("", metadata, "", listOf()))
        store.findRelevant(listOf(0.1f), 1, 0.5f)

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