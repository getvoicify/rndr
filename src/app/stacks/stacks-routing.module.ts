import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule, Routes } from '@angular/router';

const routes: Routes = [
  {
    path: '',
    loadComponent: () => import('./stacks-shell/stacks-shell.component').then((m) => m.StacksShellComponent),
    children: [
      {
        path: '',
        loadComponent: () => import('./stack-list/stack-list.component').then((m) => m.StackListComponent),
      },
      {
        path: '**',
        redirectTo: '',
      }
    ]
  },
  {
    path: '**',
    redirectTo: '',
  }
];

@NgModule({
  imports: [
    RouterModule.forChild(routes),
  ]
})
export class StacksRoutingModule {
}
