<template>
  <div class="w-full h-full" ref="editor"></div>
</template>

<script setup lang="ts">
import * as monaco from 'monaco-editor/esm/vs/editor/editor.api'
import { ref, onMounted, watch } from 'vue'
import { useDark } from '@vueuse/core'

const isDark = useDark()
const props = defineProps({
  value: {
    type: String,
    default: ''
  },
  language: {
    type: String
  },
  readonly: {
    type: Boolean
  }
})

const editorInstance = ref<monaco.editor.IStandaloneCodeEditor | null>(null)
const editor = ref<HTMLElement | null>(null)
onMounted(() => {
  if (!editor.value) return
  editorInstance.value = monaco.editor.create(editor.value, {
    value: props.value,
    language: props.language,
    readOnly: props.readonly,
    theme: isDark.value ? 'vs-dark' : 'vs-light',
    automaticLayout: true
  })
})

watch(
  () => props.value,
  () => {
    if (!editorInstance.value) return
    monaco.editor.getModel(editorInstance.value?.getModel()?.uri!)?.setValue(props.value)
  }
)
watch(
  () => props.language,
  () => {
    if (!editorInstance.value) return
    monaco.editor.setModelLanguage(editorInstance.value?.getModel()!, props.language || '')
  }
)
watch(
  () => isDark.value,
  () => monaco.editor.setTheme(isDark.value ? 'vs-dark' : 'vs-light')
)
</script>
