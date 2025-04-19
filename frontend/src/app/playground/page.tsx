import { promises as fs } from 'fs';
import path from 'path';
import Editor from './Editor';
import Markdown from 'react-markdown';
export default async function PlaygroundPage() {
  // 在服务端读取初始数据
  const markdownPath = path.join(process.cwd(), 'public', 'problemsets', '1_two-sum.md');
  const markdown = await fs.readFile(markdownPath, 'utf8');

  const codePath = path.join(process.cwd(), '..', 'templates', 'rust.rs');
  const code = await fs.readFile(codePath, 'utf8');
  return (
    <div className="grid grid-cols-2 gap-6 min-h-screen p-4">
      <div className="py-4 border rounded-lg shadow-sm">
        <div className="p-2">
          <Markdown>{markdown}</Markdown>
        </div>
      </div>

      <Editor initialCode={code} />
    </div>
  );
}