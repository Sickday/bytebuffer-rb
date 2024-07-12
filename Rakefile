require 'bundler/setup'
require 'rake'
require 'rake/testtask'
require 'rspec/core/rake_task'
require 'rubygems'
require 'rubygems/package_task'

namespace :gem do
  desc 'Build the Gem extensions'
  task :build do

  end

  desc 'Clean up the Gem build environment.'
  task :clean do
    FileUtils.rm_rf('pkg/')
    FileUtils.rm_rf('ext/target/')
  end

  desc 'Package the Gem package'
  task :package do
    load('bytebuffer-rs.gemspec')

    Gem::PackageTask.new(GEM_SPEC) do |pkg|
      pkg.need_tar_bz2 = true
    end

    Rake::Task['package'].invoke
  end
end