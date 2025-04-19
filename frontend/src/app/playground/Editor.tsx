'use client';

import { useState, useCallback, useRef, useEffect } from 'react';
import dynamic from 'next/dynamic';
interface EditorProps {
  initialCode: string;
}

// 动态导入 monaco-editor
const initMonaco = async () => {
  const monaco = await import('monaco-editor');
  return monaco;
};

export default function Editor({ initialCode }: EditorProps) {
  const [editor, setEditor] = useState<any>(null);
  const monacoEl = useRef(null);

  useEffect(() => {
    let monacoEditor: any;

    const init = async () => {
      if (monacoEl.current) {
        const monaco = await initMonaco();
        monacoEditor = monaco.editor.create(monacoEl.current, {
          value: initialCode,
          language: 'rust',
          theme: 'vs-light',
          automaticLayout: true,
          minimap: { enabled: false },
          scrollBeyondLastLine: false,
        });

        setEditor(monacoEditor);
      }
    };

    init();

    return () => {
      if (monacoEditor) {
        monacoEditor.dispose();
      }
    };
  }, [initialCode]);

  return (
    <div className="py-4 border rounded-lg shadow-sm h-full w-full">
      <div ref={monacoEl} className="w-full h-full" />
    </div>
  );
}
