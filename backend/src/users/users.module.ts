import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { UsersService } from './users.service';
import { UsersController } from './users.controller';
import { UserEntity } from './entities/user.entity';
import { TwoFactorAuthEntity } from './entities/two-factor-auth.entity';
import { UserRepository } from './user.repository';
import { UserActivityModule } from '../user-activity/user-activity.module';

@Module({
  imports: [TypeOrmModule.forFeature([UserEntity, TwoFactorAuthEntity]), UserActivityModule],
  controllers: [UsersController],
  providers: [UsersService, UserRepository],
  exports: [UsersService, UserRepository, TypeOrmModule],
})
export class UsersModule {}
