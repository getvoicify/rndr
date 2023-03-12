import { TruncateFilePathPipe } from './truncate-file-path.pipe';

describe('TruncateFilePathPipe', () => {
  let pipe: TruncateFilePathPipe;

  beforeEach(() => {
    pipe = new TruncateFilePathPipe();
  })

  it('create an instance', () => {
    expect(pipe).toBeTruthy();
  });

  describe('Extract path', () => {

    let extractFileName: (path: string) => string;

    beforeEach(() => {
      extractFileName = pipe.extractFileName;
    });

    it("should extract file name from Unix path", () => {
      expect(extractFileName("/path/to/file.txt")).toBe("file.txt");
    });

    it("should extract file name from Windows path with backslashes", () => {
      expect(extractFileName("C:\\path\\to\\file.txt")).toBe("file.txt");
    });

    it("should extract file name from Windows path with forward slashes", () => {
      expect(extractFileName("C:/path/to/file.txt")).toBe("file.txt");
    });

  });

  describe("truncateFileName", () => {
    it("should truncate file name if longer than max length", () => {
      const result = pipe.transform("/path/to/file-with-very-long-name.txt", 20);
      expect(result.length <= 24).toBe(true);
      expect(result.includes('...')).toBe(true);
    });

    it("should not truncate file name if shorter than max length", () => {
      expect(pipe.transform("/path/to/short-file.txt", 20)).toBe("short-file.txt");
    });
  });
});
