import { Pipe, PipeTransform } from '@angular/core';

@Pipe({
  name: 'truncateFilePath',
  standalone: true
})
export class TruncateFilePathPipe implements PipeTransform {

  transform(path?: string, maxLength = 0): string {

    if (!path) {
      return '';
    }

    const fileName = this.extractFileName(path);
    const extensionIndex = fileName.lastIndexOf(".");
    if (fileName.length <= maxLength) {
      return fileName;
    }
    if (extensionIndex !== -1) {
      const extension = fileName.substring(extensionIndex);
      const nameWithoutExtension = fileName.substring(0, extensionIndex);
      const halfLength = Math.floor((maxLength - extension.length) / 2);
      const beginning = nameWithoutExtension.substring(0, halfLength);
      const end = nameWithoutExtension.substring((nameWithoutExtension.length + 2) - halfLength);
      return beginning + "..." + end + extension;
    } else {
      const halfLength = Math.floor((maxLength) / 2);
      const beginning = fileName.substring(0, halfLength - 3);
      const end = fileName.substring((fileName.length) - halfLength);
      return beginning + "..." + end;
    }
  }

  extractFileName(path: string): string {
    const parts = path.split(/[\\/]/);
    return parts[parts.length - 1];
  }

}
