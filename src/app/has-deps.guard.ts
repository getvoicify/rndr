import { Router } from '@angular/router';
import { inject } from '@angular/core';
import { BridgeService, StackService } from './base/services';
import { combineLatest, map, tap } from 'rxjs';
import { AWSCredentialFormValue } from './models';

export const hasDepsGuard = () => {
  const router = inject(Router);
  const bridgeService = inject(BridgeService);
  return bridgeService.canRun$.pipe(
    map(canRun => {
      return canRun ? true : router.parseUrl('/missing-deps');
    })
  );
};

export const hasExtDepsGuard = () => {
  const router = inject(Router);
  const bridgeService = inject(BridgeService);
  return combineLatest([bridgeService.hasExtDeps$, bridgeService.extDepsGitInit$]).pipe(
    map(([hasExtDeps, extDepsGitInit]) => hasExtDeps && extDepsGitInit),
    map(hasExtDeps => {
      return hasExtDeps ? true : router.parseUrl('/missing-deps?install-ext-deps=true');
    })
  );
}

export const hasAwsEnvGuard = () => {
  const vars: (keyof AWSCredentialFormValue)[] = ['AWS_ACCESS_KEY_ID', 'AWS_SECRET_ACCESS_KEY', 'AWS_REGION'];
  const router = inject(Router);
  const bridgeService = inject(BridgeService);
  return combineLatest(vars.map(v => bridgeService.hasEnv$(v))).pipe(
    map((hasVars: boolean[]) => hasVars.every(hasVar => hasVar)),
    map(hasVars => hasVars ? true : router.parseUrl('/missing-vars'))
  );
};

export const hasStacksGuard = () => {
  const router = inject(Router);
  const stackService = inject(StackService);
  return stackService.hasStacks$.pipe(
    map(hasStacks => hasStacks ? true : router.parseUrl('/stacks'))
  );
}
