/* tslint:disable:no-unused-variable */

import { TestBed, async, inject } from '@angular/core/testing';
import { ValidateVotecodeService } from './ValidateVotecode.service';

describe('Service: ValidateVotecode', () => {
  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [ValidateVotecodeService]
    });
  });

  it('should ...', inject([ValidateVotecodeService], (service: ValidateVotecodeService) => {
    expect(service).toBeTruthy();
  }));
});
